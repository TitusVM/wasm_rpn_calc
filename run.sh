#!/bin/bash

# This script is used after the components have been built and signed with both the safe and vulnerable versions of the dependencies.
# The script uses a custom version of the wasmtime runtime to run the composed components and perform some checks.

# Exit the script if any command fails
set -e

cosign verify ghcr.io/titusvm/wasm_rpn_calc_test:latest --certificate-identity=tvmab@pm.me --certificate-oidc-issuer=https://github.com/login/oauth

# Check whether or not the CR_PAT environment variable is set and ask for it if it is not
# if [ -z "$CR_PAT" ]; then
#     echo "Please provide a personal access token:"
#     read CR_PAT
# fi

#  -u TitusVM -p $CR_PAT
wkg oci pull ghcr.io/titusvm/wasm_rpn_calc_test:latest

~/master-work/pr/wasmtime/target/debug/wasmtime --audit -k ./publisher-key/public.key titusvm_wasm_rpn_calc_test.wasm