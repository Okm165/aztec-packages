use std::ptr;

use super::{
    acir_create_circuit, acir_create_proof, acir_delete_acir_composer, acir_get_circuit_sizes,
    acir_get_proving_key, acir_get_verification_key, acir_init_proving_key,
    acir_init_verification_key, acir_load_verification_key, acir_new_acir_composer,
    acir_new_goblin_acir_composer, acir_verify_proof, models::Ptr, traits::SerializeBuffer, Buffer,
};

pub unsafe fn api_get_circuit_sizes(constraint_system_buf: &[u8]) -> (u32, u32, u32) {
    let mut exact = 0;
    let mut total = 0;
    let mut subgroup = 0;
    acir_get_circuit_sizes(
        constraint_system_buf.to_buffer().as_slice().as_ptr(),
        &mut exact,
        &mut total,
        &mut subgroup,
    );
    (exact, total, subgroup)
}

pub unsafe fn api_new_acir_composer(size_hint: u32) -> Ptr {
    let mut out_ptr = ptr::null_mut();
    acir_new_acir_composer(size_hint.to_be_bytes().as_ptr() as *const u32, &mut out_ptr);
    out_ptr
}

pub unsafe fn api_new_goblin_acir_composer() -> Ptr {
    let mut out_ptr = ptr::null_mut();
    acir_new_goblin_acir_composer(&mut out_ptr);
    out_ptr
}

pub unsafe fn api_delete_acir_composer(acir_composer_ptr: Ptr) {
    acir_delete_acir_composer(&acir_composer_ptr);
}

pub unsafe fn api_acir_create_circuit(
    acir_composer_ptr: &mut Ptr,
    constraint_system_buf: &[u8],
    size_hint: u32,
) {
    acir_create_circuit(
        acir_composer_ptr,
        constraint_system_buf.to_buffer().as_slice().as_ptr(),
        size_hint.to_be_bytes().as_ptr() as *const u32,
    );
}

pub unsafe fn api_acir_init_proving_key(
    acir_composer_ptr: &mut Ptr,
    constraint_system_buf: &[u8],
) {
    acir_init_proving_key(
        acir_composer_ptr,
        constraint_system_buf.to_buffer().as_slice().as_ptr(),
    );
}

pub unsafe fn api_acir_create_proof(
    acir_composer_ptr: &mut Ptr,
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
) -> Vec<u8> {
    let mut out_ptr = ptr::null_mut();
    acir_create_proof(
        acir_composer_ptr,
        constraint_system_buf.to_buffer().as_slice().as_ptr(),
        witness_buf.to_buffer().as_slice().as_ptr(),
        &mut out_ptr,
    );
    Buffer::from_ptr(out_ptr).unwrap().to_vec()
}

pub unsafe fn api_acir_load_verification_key(acir_composer_ptr: &mut Ptr, vk_buf: &[u8]) {
    acir_load_verification_key(acir_composer_ptr, vk_buf.to_buffer().as_slice().as_ptr());
}

pub unsafe fn api_acir_init_verification_key(acir_composer_ptr: &mut Ptr) {
    acir_init_verification_key(acir_composer_ptr);
}

pub unsafe fn api_acir_get_verification_key(acir_composer_ptr: &mut Ptr) -> Vec<u8> {
    let mut out_ptr = ptr::null_mut();
    acir_get_verification_key(acir_composer_ptr, &mut out_ptr);
    Buffer::from_ptr(out_ptr).unwrap().to_vec()
}

pub unsafe fn api_acir_get_proving_key(acir_composer_ptr: &mut Ptr, acir_vec: &[u8]) -> Vec<u8> {
    let mut out_ptr = ptr::null_mut();
    acir_get_proving_key(
        acir_composer_ptr,
        acir_vec.to_buffer().as_slice().as_ptr(),
        &mut out_ptr,
    );
    Buffer::from_ptr(out_ptr).unwrap().to_vec()
}

pub unsafe fn api_acir_verify_proof(acir_composer_ptr: &mut Ptr, proof_buf: &[u8]) -> bool {
    let mut result = false;
    acir_verify_proof(
        acir_composer_ptr,
        proof_buf.to_buffer().as_slice().as_ptr(),
        &mut result,
    );
    result
}
