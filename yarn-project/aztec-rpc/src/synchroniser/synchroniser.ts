import { Tx, TxHash } from '@aztec/tx';
import { AztecNode } from '@aztec/aztec-node';
import { createDebugLogger, InterruptableSleep } from '@aztec/foundation';
import { AccountState } from '../account_state/index.js';
import {
  AztecAddress,
  EthAddress,
  KERNEL_NEW_COMMITMENTS_LENGTH,
  KERNEL_NEW_CONTRACTS_LENGTH,
  KERNEL_NEW_NULLIFIERS_LENGTH,
} from '@aztec/circuits.js';
import { Database, TxDao } from '../database/index.js';
import { ContractAbi } from '../noir.js';
import { L2Block } from '@aztec/l2-block';
import { keccak } from '@aztec/foundation';
import { TxReceipt, TxStatus } from '../tx/index.js';

export class Synchroniser {
  private runningPromise?: Promise<void>;
  private accountStates: AccountState[] = [];
  private interruptableSleep = new InterruptableSleep();
  private running = false;

  constructor(
    private node: AztecNode,
    private db: Database,
    private log = createDebugLogger('aztec:aztec_rps_synchroniser'),
  ) {}

  public start(from = 1, take = 1, retryInterval = 1000) {
    if (this.running) {
      return;
    }

    this.running = true;

    const run = async () => {
      while (this.running) {
        try {
          const blocks = await this.node.getBlocks(from, take);

          if (!blocks.length) {
            await this.interruptableSleep.sleep(retryInterval);
            continue;
          }

          await this.decodeBlocks(blocks);

          from += blocks.length;
        } catch (err) {
          console.log(err);
          await this.interruptableSleep.sleep(retryInterval);
        }
      }
    };

    this.runningPromise = run();
    this.log('Started');
  }

  public async stop() {
    this.running = false;
    this.interruptableSleep.interrupt();
    await this.runningPromise;
    this.log('Stopped');
  }

  public async addAccount(account: AztecAddress) {
    this.accountStates.push(new AccountState(account, this.db));
    await Promise.resolve();
  }

  public getAccount(account: AztecAddress) {
    return this.accountStates.find(as => as.publicKey.toBuffer().equals(account.toBuffer()));
  }

  public getAccounts() {
    return [...this.accountStates];
  }

  public async addPendingContractAbi(contractAddress: AztecAddress, portalContract: EthAddress, abi: ContractAbi) {
    await this.db.addContract(contractAddress, portalContract, abi, false);
  }

  public async getTxByHash(txHash: TxHash): Promise<TxDao | undefined> {
    const tx = await this.db.getTx(txHash);

    if (!tx) {
      return;
    }

    const account = this.getAccount(tx.from);
    if (!account) {
      throw new Error('Unauthorised account.');
    }

    return tx;
  }

  private async decodeBlocks(l2Blocks: L2Block[]) {
    for (const block of l2Blocks) {
      let i = 0;
      const numTxs = Math.floor(block.newCommitments.length / KERNEL_NEW_COMMITMENTS_LENGTH);
      while (i < numTxs) {
        const dataToHash = Buffer.concat(
          [
            block.newCommitments
              .slice(
                i * KERNEL_NEW_COMMITMENTS_LENGTH,
                i * KERNEL_NEW_COMMITMENTS_LENGTH + KERNEL_NEW_COMMITMENTS_LENGTH,
              )
              .map(x => x.toBuffer()),
            block.newNullifiers
              .slice(i * KERNEL_NEW_NULLIFIERS_LENGTH, i * KERNEL_NEW_NULLIFIERS_LENGTH + KERNEL_NEW_NULLIFIERS_LENGTH)
              .map(x => x.toBuffer()),
            block.newContracts
              .slice(i * KERNEL_NEW_CONTRACTS_LENGTH, i * KERNEL_NEW_CONTRACTS_LENGTH + KERNEL_NEW_CONTRACTS_LENGTH)
              .map(x => x.toBuffer()),
          ].flat(),
        );
        const txDao: TxDao | undefined = await this.db.getTx(new TxHash(keccak(dataToHash)));
        if (txDao !== undefined) {
          txDao.blockHash = keccak(block.encode());
          txDao.blockNumber = block.number;
          await this.db.addOrUpdateTx(txDao);
        }
        i++;
      }
      this.log(`Synched block ${block.number}`);
    }
  }
}
