import {
  makeBaseParityInputs,
  makeBaseRollupInputs,
  makeParityPublicInputs,
  makeProof,
} from '@aztec/circuits.js/testing';

import { MemoryProvingQueue } from './memory-proving-queue.js';
import { type ProvingQueue } from './proving-queue.js';
import { ProvingRequestType } from './proving-request.js';

describe('MemoryProvingQueue', () => {
  let queue: ProvingQueue;

  beforeEach(() => {
    queue = new MemoryProvingQueue();
  });

  it('returns jobs in order', async () => {
    void queue.prove({
      type: ProvingRequestType.BASE_PARITY,
      inputs: makeBaseParityInputs(),
    });

    void queue.prove({
      type: ProvingRequestType.BASE_ROLLUP,
      inputs: makeBaseRollupInputs(),
    });

    const job1 = await queue.getProvingJob();
    expect(job1?.request.type).toEqual(ProvingRequestType.BASE_PARITY);

    const job2 = await queue.getProvingJob();
    expect(job2?.request.type).toEqual(ProvingRequestType.BASE_ROLLUP);
  });

  it('returns null when no jobs are available', async () => {
    await expect(queue.getProvingJob({ timeoutSec: 0 })).resolves.toBeNull();
  });

  it('notifies of completion', async () => {
    const inputs = makeBaseParityInputs();
    const promise = queue.prove({
      inputs,
      type: ProvingRequestType.BASE_PARITY,
    });

    const job = await queue.getProvingJob();
    expect(job?.request.inputs).toEqual(inputs);

    const publicInputs = makeParityPublicInputs();
    const proof = makeProof();
    await queue.resolveProvingJob(job!.id, [publicInputs, proof]);
    await expect(promise).resolves.toEqual([publicInputs, proof]);
  });

  it('notifies of errors', async () => {
    const inputs = makeBaseParityInputs();
    const promise = queue.prove({
      inputs,
      type: ProvingRequestType.BASE_PARITY,
    });
    const job = await queue.getProvingJob();
    expect(job?.request.inputs).toEqual(inputs);

    const error = new Error('test error');
    await queue.rejectProvingJob(job!.id, error);
    await expect(promise).rejects.toEqual(error);
  });
});
