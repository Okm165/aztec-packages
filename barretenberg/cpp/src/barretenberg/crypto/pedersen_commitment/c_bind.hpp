#pragma once
#include "barretenberg/common/wasm_export.hpp"
#include "barretenberg/ecc/curves/bn254/fr.hpp"
#include "barretenberg/ecc/curves/grumpkin/grumpkin.hpp"

WASM_EXPORT void pedersen_commit(const uint8_t* inputs_buffer, uint8_t* output);