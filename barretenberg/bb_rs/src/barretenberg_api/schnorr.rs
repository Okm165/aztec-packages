use super::{
    models::{Fq, Fr, Point},
    schnorr_compute_public_key, schnorr_construct_signature,
    schnorr_multisig_create_multisig_public_key, schnorr_negate_public_key,
    schnorr_verify_signature,
    traits::{DeserializeBuffer, SerializeBuffer},
};

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

pub unsafe fn api_schnorr_multisig_create_multisig_public_key(public_key: &Fq) -> [u8; 128] {
    let mut result = [0; 128];
    schnorr_multisig_create_multisig_public_key(
        public_key.to_buffer().as_slice().as_ptr(),
        result.as_mut_ptr(),
    );
    result
}
