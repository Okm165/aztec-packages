use dep::protocol_types::{address::AztecAddress, grumpkin_point::GrumpkinPoint};

// TODO: Should take encrypted data.
#[oracle(emitEncryptedLog)]
fn emit_encrypted_log_oracle<N>(
    _contract_address: AztecAddress,
    _storage_slot: Field,
    _note_type_id: Field,
    _encryption_pub_key: GrumpkinPoint,
    _preimage: [Field; N]
) -> Field {}

unconstrained pub fn emit_encrypted_log<N>(
    contract_address: AztecAddress,
    storage_slot: Field,
    note_type_id: Field,
    encryption_pub_key: GrumpkinPoint,
    preimage: [Field; N]
) -> Field {
    emit_encrypted_log_oracle(
        contract_address,
        storage_slot,
        note_type_id,
        encryption_pub_key,
        preimage
    )
}
