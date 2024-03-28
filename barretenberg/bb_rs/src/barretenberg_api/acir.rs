use super::{bindgen, models::Ptr, traits::SerializeBuffer, Buffer};
use std::ptr;

#[derive(Debug)]
pub struct CircuitSizes {
    pub exact: u32,
    pub total: u32,
    pub subgroup: u32,
}

pub unsafe fn get_circuit_sizes(constraint_system_buf: &[u8]) -> CircuitSizes {
    let mut exact = 0;
    let mut total = 0;
    let mut subgroup = 0;
    bindgen::acir_get_circuit_sizes(
        constraint_system_buf.to_buffer().as_slice().as_ptr(),
        &mut exact,
        &mut total,
        &mut subgroup,
    );
    CircuitSizes {
        exact: exact.to_be(),
        total: total.to_be(),
        subgroup: subgroup.to_be(),
    }
}

pub unsafe fn new_acir_composer(size_hint: u32) -> Ptr {
    let mut out_ptr = ptr::null_mut();
    bindgen::acir_new_acir_composer(size_hint.to_be_bytes().as_ptr() as *const u32, &mut out_ptr);
    out_ptr
}

pub unsafe fn new_goblin_acir_composer() -> Ptr {
    let mut out_ptr = ptr::null_mut();
    bindgen::acir_new_goblin_acir_composer(&mut out_ptr);
    out_ptr
}

pub unsafe fn delete_acir_composer(acir_composer_ptr: Ptr) {
    bindgen::acir_delete_acir_composer(&acir_composer_ptr);
}

pub unsafe fn acir_create_circuit(
    acir_composer_ptr: &mut Ptr,
    constraint_system_buf: &[u8],
    size_hint: u32,
) {
    bindgen::acir_create_circuit(
        acir_composer_ptr,
        constraint_system_buf.to_buffer().as_slice().as_ptr(),
        size_hint.to_be_bytes().as_ptr() as *const u32,
    );
}

pub unsafe fn acir_init_proving_key(acir_composer_ptr: &mut Ptr, constraint_system_buf: &[u8]) {
    bindgen::acir_init_proving_key(
        acir_composer_ptr,
        constraint_system_buf.to_buffer().as_slice().as_ptr(),
    );
}

pub unsafe fn acir_create_proof(
    acir_composer_ptr: &mut Ptr,
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
) -> Vec<u8> {
    let mut out_ptr = ptr::null_mut();
    bindgen::acir_create_proof(
        acir_composer_ptr,
        constraint_system_buf.to_buffer().as_slice().as_ptr(),
        witness_buf.to_buffer().as_slice().as_ptr(),
        &mut out_ptr,
    );
    Buffer::from_ptr(
        Buffer::from_ptr(out_ptr)
            .unwrap()
            .to_vec()
            .as_slice()
            .as_ptr(),
    )
    .unwrap()
    .to_vec()
}

pub unsafe fn acir_load_verification_key(acir_composer_ptr: &mut Ptr, vk_buf: &[u8]) {
    bindgen::acir_load_verification_key(acir_composer_ptr, vk_buf.as_ptr());
}

pub unsafe fn acir_init_verification_key(acir_composer_ptr: &mut Ptr) {
    bindgen::acir_init_verification_key(acir_composer_ptr);
}

pub unsafe fn acir_get_verification_key(acir_composer_ptr: &mut Ptr) -> Vec<u8> {
    let mut out_ptr = ptr::null_mut();
    bindgen::acir_get_verification_key(acir_composer_ptr, &mut out_ptr);
    Buffer::from_ptr(
        Buffer::from_ptr(out_ptr)
            .unwrap()
            .to_vec()
            .as_slice()
            .as_ptr(),
    )
    .unwrap()
    .to_vec()
}

pub unsafe fn acir_get_proving_key(acir_composer_ptr: &mut Ptr, acir_vec: &[u8]) -> Vec<u8> {
    let mut out_ptr = ptr::null_mut();
    bindgen::acir_get_proving_key(
        acir_composer_ptr,
        acir_vec.to_buffer().as_slice().as_ptr(),
        &mut out_ptr,
    );
    Buffer::from_ptr(out_ptr).unwrap().to_vec()
}

pub unsafe fn acir_verify_proof(acir_composer_ptr: &mut Ptr, proof_buf: &[u8]) -> bool {
    let mut result = false;
    bindgen::acir_verify_proof(
        acir_composer_ptr,
        proof_buf.to_buffer().as_ptr(),
        &mut result,
    );
    result
}
