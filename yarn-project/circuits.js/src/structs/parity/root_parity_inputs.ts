import { BufferReader, type Tuple, serializeToBuffer } from '@aztec/foundation/serialize';

import { NUM_BASE_PARITY_PER_ROOT_PARITY } from '../../constants.gen.js';
import { RootParityInput } from './root_parity_input.js';

export class RootParityInputs {
  constructor(
    /** Public inputs of children and their proofs. */
    public readonly children: Tuple<RootParityInput, typeof NUM_BASE_PARITY_PER_ROOT_PARITY>,
  ) {}

  /**
   * Serializes the inputs to a buffer.
   * @returns The inputs serialized to a buffer.
   */
  toBuffer() {
    return serializeToBuffer(this.children);
  }

  /**
   * Serializes the inputs to a hex string.
   * @returns The instance serialized to a hex string.
   */
  toString() {
    return this.toBuffer().toString('hex');
  }

  /**
   * Deserializes the inputs from a buffer.
   * @param buffer - The buffer to deserialize from.
   * @returns A new RootParityInputs instance.
   */
  static fromBuffer(buffer: Buffer | BufferReader) {
    const reader = BufferReader.asReader(buffer);
    return new RootParityInputs(reader.readArray(NUM_BASE_PARITY_PER_ROOT_PARITY, RootParityInput));
  }

  /**
   * Deserializes the inputs from a hex string.
   * @param str - A hex string to deserialize from.
   * @returns A new RootParityInputs instance.
   */
  static fromString(str: string) {
    return RootParityInputs.fromBuffer(Buffer.from(str, 'hex'));
  }
}
