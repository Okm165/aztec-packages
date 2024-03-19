use crate::barretenberg_api::{
    pedersen_commit, pedersen_hash, pedersen_hashes, traits::DeserializeBuffer,
};

use super::{
    models::{Fr, Point},
    traits::SerializeBuffer,
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
