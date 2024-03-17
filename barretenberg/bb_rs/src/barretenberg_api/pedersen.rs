use crate::barretenberg_api::{
    pedersen_commit, pedersen_hash, pedersen_hashes, poseidon_hash, poseidon_hashes,
    traits::{DeserializeBuffer, SerializeBuffer},
};

use super::{
    blake2s, blake2s_to_field_,
    models::{Fr, Point},
    schnorr_compute_public_key, schnorr_construct_signature, schnorr_negate_public_key,
    schnorr_verify_signature,
};

pub unsafe fn api_pedersen_commit(inputs: &[Fr]) -> Point {
    let mut output: <Point as DeserializeBuffer>::Slice = [0; 64];
    pedersen_commit(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr());
    Point::from_buffer(output)
}

pub unsafe fn api_pedersen_hash(inputs: &[Fr], hash_index: u32) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    pedersen_hash(
        inputs.to_buffer().as_slice().as_ptr(),
        hash_index.to_be_bytes().as_ptr() as *const u32,
        output.as_mut_ptr(),
    );
    Fr::from_buffer(output)
}

pub unsafe fn api_pedersen_hashes(inputs: &[Fr], hash_index: u32) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    pedersen_hashes(
        inputs.to_buffer().as_slice().as_ptr(),
        hash_index.to_be_bytes().as_ptr() as *const u32,
        output.as_mut_ptr(),
    );
    Fr::from_buffer(output)
}

pub unsafe fn api_pedersen_hash_buffer(inputs: &[u8], hash_index: u32) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    pedersen_hashes(
        inputs.as_ptr(),
        hash_index.to_be_bytes().as_ptr() as *const u32,
        output.as_mut_ptr(),
    );
    Fr::from_buffer(output)
}

pub unsafe fn api_poseidon_hash(inputs: &[Fr]) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    poseidon_hash(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr());
    Fr::from_buffer(output)
}

pub unsafe fn api_poseidon_hashes(inputs: &[Fr]) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    poseidon_hashes(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr());
    Fr::from_buffer(output)
}

pub unsafe fn api_blake2s(inputs: &[u8]) -> [u8; 32] {
    let mut output = [0; 32];
    blake2s(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr());
    output
}

pub unsafe fn api_blake2s_to_field(inputs: &[u8]) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    blake2s_to_field_(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr());
    Fr::from_buffer(output)
}

pub unsafe fn api_schnorr_compute_public_key(private_key: &Fr) -> Point {
    let mut output: <Point as DeserializeBuffer>::Slice = [0; 64];
    schnorr_compute_public_key(
        private_key.to_buffer().as_slice().as_ptr(),
        output.as_mut_ptr(),
    );
    Point::from_buffer(output)
}

pub unsafe fn api_schnorr_negate_public_key(public_key: &Point) -> Point {
    let mut output: <Point as DeserializeBuffer>::Slice = [0; 64];
    schnorr_negate_public_key(
        public_key.to_buffer().as_slice().as_ptr(),
        output.as_mut_ptr(),
    );
    Point::from_buffer(output)
}

pub unsafe fn api_schnorr_construct_signature(
    message: &[u8],
    private_key: &Fr,
) -> ([u8; 32], [u8; 32]) {
    let mut s = [0; 32];
    let mut e = [0; 32];
    schnorr_construct_signature(
        message.to_buffer().as_slice().as_ptr(),
        private_key.to_buffer().as_slice().as_ptr(),
        s.as_mut_ptr(),
        e.as_mut_ptr(),
    );
    (s, e)
}

pub unsafe fn api_schnorr_verify_signature(
    message: &[u8],
    public_key: &Point,
    sig_s: &mut [u8; 32],
    sig_e: &mut [u8; 32],
) -> bool {
    let mut result = false;
    schnorr_verify_signature(
        message.to_buffer().as_slice().as_ptr(),
        public_key.to_buffer().as_slice().as_ptr(),
        sig_s.as_mut_ptr(),
        sig_e.as_mut_ptr(),
        &mut result,
    );
    result
}
