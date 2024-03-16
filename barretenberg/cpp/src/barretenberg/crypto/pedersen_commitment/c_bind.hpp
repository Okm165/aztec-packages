#pragma once
#include "barretenberg/common/wasm_export.hpp"
#include "barretenberg/ecc/curves/bn254/fr.hpp"
#include "barretenberg/ecc/curves/grumpkin/grumpkin.hpp"

WASM_EXPORT void pedersen_commit(bb::fr::vec_in_buf inputs_buffer, bb::fr::out_buf output);