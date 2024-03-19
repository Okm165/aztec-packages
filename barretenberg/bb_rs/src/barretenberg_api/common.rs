use super::examples_simple_create_and_verify_proof;

pub unsafe fn api_example_simple_create_and_verify_proof() -> bool {
    let mut result = false;
    examples_simple_create_and_verify_proof(&mut result);
    result
}
