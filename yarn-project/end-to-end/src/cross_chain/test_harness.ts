import { AztecNodeService } from '@aztec/aztec-node';
import { Contract, Wallet, computeMessageSecretHash } from '@aztec/aztec.js';
import { AztecAddress, EthAddress, Fr, Point } from '@aztec/circuits.js';
import { DeployL1Contracts } from '@aztec/ethereum';
import { DebugLogger } from '@aztec/foundation/log';
import { OutboxAbi } from '@aztec/l1-artifacts';
import { sha256ToField } from '@aztec/foundation/crypto';
import { toBufferBE } from '@aztec/foundation/bigint-buffer';
import { AztecRPCServer } from '@aztec/aztec-rpc';
import { TxStatus } from '@aztec/types';
import { PublicClient, HttpTransport, Chain, getContract } from 'viem';

import { deployAndInitializeNonNativeL2TokenContracts, expectAztecStorageSlot } from '../utils.js';

/**
 * A Class for testing cross chain interactions, contains common interactions
 * shared between cross chain tests.
 */
export class CrossChainTestHarness {
  static async new(
    initialBalance: bigint,
    aztecNode: AztecNodeService,
    aztecRpcServer: AztecRPCServer,
    deployL1ContractsValues: DeployL1Contracts,
    accounts: AztecAddress[],
    wallet: Wallet,
    logger: DebugLogger,
  ): Promise<CrossChainTestHarness> {
    const walletClient = deployL1ContractsValues.walletClient;
    const publicClient = deployL1ContractsValues.publicClient;

    const ethAccount = EthAddress.fromString((await walletClient.getAddresses())[0]);
    const [ownerAddress, receiver] = accounts;
    const ownerPub = await aztecRpcServer.getAccountPublicKey(ownerAddress);

    const outbox = getContract({
      address: deployL1ContractsValues.outboxAddress.toString(),
      abi: OutboxAbi,
      publicClient,
    });

    // Deploy and initialize all required contracts
    logger('Deploying Portal, initializing and deploying l2 contract...');
    const contracts = await deployAndInitializeNonNativeL2TokenContracts(
      wallet,
      walletClient,
      publicClient,
      deployL1ContractsValues!.registryAddress,
      initialBalance,
      ownerPub.toBigInts(),
    );
    const l2Contract = contracts.l2Contract;
    const underlyingERC20 = contracts.underlyingERC20;
    const tokenPortal = contracts.tokenPortal;
    const tokenPortalAddress = contracts.tokenPortalAddress;
    // await expectBalance(accounts[0], initialBalance);
    logger('Successfully deployed contracts and initialized portal');

    return new CrossChainTestHarness(
      aztecNode,
      aztecRpcServer,
      accounts,
      logger,
      l2Contract,
      ethAccount,
      tokenPortalAddress,
      tokenPortal,
      underlyingERC20,
      outbox,
      publicClient,
      walletClient,
      ownerAddress,
      receiver,
      ownerPub,
    );
  }
  constructor(
    /** AztecNode. */
    public aztecNode: AztecNodeService,
    /** AztecRpcServer. */
    public aztecRpcServer: AztecRPCServer,
    /** Accounts. */
    public accounts: AztecAddress[],
    /** Logger. */
    public logger: DebugLogger,

    /** Testing aztec contract. */
    public l2Contract: Contract,
    /** Eth account to interact with. */
    public ethAccount: EthAddress,

    /** Portal address. */
    public tokenPortalAddress: EthAddress,
    /** Token portal instance. */
    public tokenPortal: any,
    /** Underlying token for portal tests. */
    public underlyingERC20: any,
    /** Message Bridge Outbox. */
    public outbox: any,
    /** Viem Public client instance. */
    public publicClient: PublicClient<HttpTransport, Chain>,
    /** Viem Walllet Client instance. */
    public walletClient: any,

    /** Aztec address to use in tests. */
    public ownerAddress: AztecAddress,
    /** Another Aztec Address to use in tests. */
    public receiver: AztecAddress,
    /** The owners public key. */
    public ownerPub: Point,
  ) {}

  async generateClaimSecret(): Promise<[Fr, Fr]> {
    this.logger("Generating a claim secret using pedersen's hash function");
    const secret = Fr.random();
    const secretHash = await computeMessageSecretHash(secret);
    this.logger('Generated claim secret: ', secretHash.toString(true));
    return [secret, secretHash];
  }

  async mintTokensOnL1(amount: bigint) {
    this.logger('Minting tokens on L1');
    await this.underlyingERC20.write.mint([this.ethAccount.toString(), amount], {} as any);
    expect(await this.underlyingERC20.read.balanceOf([this.ethAccount.toString()])).toBe(amount);
  }

  async getL1BalanceOf(address: EthAddress) {
    return await this.underlyingERC20.read.balanceOf([address.toString()]);
  }

  async sendTokensToPortal(bridgeAmount: bigint, secretHash: Fr) {
    await this.underlyingERC20.write.approve([this.tokenPortalAddress.toString(), bridgeAmount], {} as any);

    // Deposit tokens to the TokenPortal
    const deadline = 2 ** 32 - 1; // max uint32 - 1

    this.logger('Sending messages to L1 portal to be consumed privately');
    const args = [
      this.ownerAddress.toString(),
      bridgeAmount,
      deadline,
      secretHash.toString(true),
      this.ethAccount.toString(),
    ] as const;
    const { result: messageKeyHex } = await this.tokenPortal.simulate.depositToAztec(args, {
      account: this.ethAccount.toString(),
    } as any);
    await this.tokenPortal.write.depositToAztec(args, {} as any);

    return Fr.fromString(messageKeyHex);
  }

  async performL2Transfer(transferAmount: bigint) {
    // send a transfer tx to force through rollup with the message included
    const transferTx = this.l2Contract.methods
      .transfer(
        transferAmount,
        (await this.aztecRpcServer.getAccountPublicKey(this.ownerAddress)).toBigInts(),
        (await this.aztecRpcServer.getAccountPublicKey(this.receiver)).toBigInts(),
      )
      .send({ from: this.accounts[0] });

    await transferTx.isMined(0, 0.1);
    const transferReceipt = await transferTx.getReceipt();

    expect(transferReceipt.status).toBe(TxStatus.MINED);
  }

  async consumeMessageOnAztecAndMintSecretly(bridgeAmount: bigint, messageKey: Fr, secret: Fr) {
    this.logger('Consuming messages on L2 secretively');
    // Call the mint tokens function on the noir contract
    const consumptionTx = this.l2Contract.methods
      .mint(bridgeAmount, this.ownerPub, this.ownerAddress, messageKey, secret, this.ethAccount.toField())
      .send({ from: this.ownerAddress });

    await consumptionTx.isMined(0, 0.1);
    const consumptionReceipt = await consumptionTx.getReceipt();
    expect(consumptionReceipt.status).toBe(TxStatus.MINED);
  }

  async consumeMessageOnAztecAndMintPublicly(bridgeAmount: bigint, messageKey: Fr, secret: Fr) {
    this.logger('Consuming messages on L2 Publicly');
    // Call the mint tokens function on the noir contract
    const consumptionTx = this.l2Contract.methods
      .mintPublic(bridgeAmount, this.ownerAddress, messageKey, secret, this.ethAccount.toField())
      .send({ from: this.ownerAddress });

    await consumptionTx.isMined(0, 0.1);
    const consumptionReceipt = await consumptionTx.getReceipt();
    expect(consumptionReceipt.status).toBe(TxStatus.MINED);
  }

  async getL2BalanceOf(owner: AztecAddress) {
    const ownerPublicKey = await this.aztecRpcServer.getAccountPublicKey(owner);
    const [balance] = await this.l2Contract.methods.getBalance(ownerPublicKey.toBigInts()).view({ from: owner });
    return balance;
  }

  async expectBalanceOnL2(owner: AztecAddress, expectedBalance: bigint) {
    const balance = await this.getL2BalanceOf(owner);
    this.logger(`Account ${owner} balance: ${balance}`);
    expect(balance).toBe(expectedBalance);
  }

  async expectPublicBalanceOnL2(owner: AztecAddress, expectedBalance: bigint, publicBalanceSlot: bigint) {
    await expectAztecStorageSlot(
      this.logger,
      this.aztecNode,
      this.l2Contract,
      publicBalanceSlot,
      owner.toField(),
      expectedBalance,
    );
  }

  async checkEntryIsNotInOutbox(withdrawAmount: bigint, callerOnL1: EthAddress = EthAddress.ZERO): Promise<Fr> {
    this.logger('Ensure that the entry is not in outbox yet');
    const contractInfo = await this.aztecNode.getContractInfo(this.l2Contract.address);
    // 0xb460af94, selector for "withdraw(uint256,address,address)"
    const content = sha256ToField(
      Buffer.concat([
        Buffer.from([0xb4, 0x60, 0xaf, 0x94]),
        toBufferBE(withdrawAmount, 32),
        this.ethAccount.toBuffer32(),
        callerOnL1.toBuffer32(),
      ]),
    );
    const entryKey = sha256ToField(
      Buffer.concat([
        this.l2Contract.address.toBuffer(),
        new Fr(1).toBuffer(), // aztec version
        contractInfo?.portalContractAddress.toBuffer32() ?? Buffer.alloc(32, 0),
        new Fr(this.publicClient.chain.id).toBuffer(), // chain id
        content.toBuffer(),
      ]),
    );
    expect(await this.outbox.read.contains([entryKey.toString(true)])).toBeFalsy();

    return entryKey;
  }

  async withdrawFundsFromBridgeOnL1(withdrawAmount: bigint, entryKey: Fr) {
    this.logger('Send L1 tx to consume entry and withdraw funds');
    // Call function on L1 contract to consume the message
    const { request: withdrawRequest, result: withdrawEntryKey } = await this.tokenPortal.simulate.withdraw([
      withdrawAmount,
      this.ethAccount.toString(),
      false,
    ]);

    expect(withdrawEntryKey).toBe(entryKey.toString(true));
    expect(await this.outbox.read.contains([withdrawEntryKey])).toBeTruthy();

    await this.walletClient.writeContract(withdrawRequest);
    return withdrawEntryKey;
  }

  async shieldFundsOnL2(shieldAmount: bigint, secretHash: Fr) {
    this.logger('Shielding funds on L2');
    const shieldTx = this.l2Contract.methods.shield(shieldAmount, secretHash).send({ from: this.ownerAddress });
    await shieldTx.isMined(0, 0.1);
    const shieldReceipt = await shieldTx.getReceipt();
    expect(shieldReceipt.status).toBe(TxStatus.MINED);
  }

  async redeemShieldPrivatelyOnL2(shieldAmount: bigint, secret: Fr) {
    this.logger('Spending commitment in private call');
    const privateTx = this.l2Contract.methods
      .redeemShield(shieldAmount, secret, this.ownerPub)
      .send({ from: this.ownerAddress });

    await privateTx.isMined();
    const privateReceipt = await privateTx.getReceipt();

    expect(privateReceipt.status).toBe(TxStatus.MINED);
  }

  async unshieldTokensOnL2(unshieldAmount: bigint) {
    this.logger('Unshielding tokens');
    const unshieldTx = this.l2Contract.methods
      .unshieldTokens(unshieldAmount, this.ownerPub, this.ownerAddress.toField())
      .send({ from: this.ownerAddress });
    await unshieldTx.isMined();
    const unshieldReceipt = await unshieldTx.getReceipt();

    expect(unshieldReceipt.status).toBe(TxStatus.MINED);
  }

  async stop() {
    await this.aztecNode?.stop();
    await this.aztecRpcServer?.stop();
  }
}
