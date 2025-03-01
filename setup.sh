#!/bin/bash

# This script is used to build and sign the components with both the safe and vulnerable versions of the dependencies.
# The script performs the following steps:
# 1. Generate signing keys
# 2. Build and sign the components with the original Cargo.toml
# 3. Switch to the vulnerable version of the dependencies
# 4. Build and sign the components with the VulnerableCargo.toml
# 5. Restore the original Cargo.toml

# Step 1: Generate signing keys
echo "Generating signing keys..."
wasmsign2 keygen --public-key public.key --secret-key secret.key 

mkdir -p binaries

build_and_sign_components() {
  local suffix=$1
  
  for component in rpn command; do
    echo "Processing component: $component ($suffix)"
    
    cd $component

    echo "Checking for unreviewed dependencies..."
    cargo vet --output-format=json > ${component}_${suffix}.json
    
    echo "Building component..."
    cargo auditable build --target=wasm32-wasip2

    wasmadder add -i target/wasm32-wasip2/debug/$component.wasm -d ${component}_${suffix}.json -s .vet-v0 -o target/wasm32-wasip2/debug/${component}.wasm

    echo "Signing component..."
    SIGNED_WASM="../binaries/signed_${component}_${suffix}.wasm"
    wasmsign2 sign -k ../secret.key -i target/wasm32-wasip2/debug/$component.wasm -o $SIGNED_WASM  > /dev/null

    cd ..

    echo "Composing components ($suffix)..."
    COMPOSED_WASM="binaries/composed_${suffix}.wasm"
    wac plug binaries/signed_command_${suffix}.wasm --plug binaries/signed_rpn_${suffix}.wasm -o $COMPOSED_WASM
    
    SIGNED_COMPOSED_WASM="signed_composed_${suffix}.wasm"
    echo "Signing composed component ($suffix)..."
    wasmsign2 sign -k secret.key -i $COMPOSED_WASM -o $SIGNED_COMPOSED_WASM > /dev/null

  done
}

# Step 2: Process original Cargo.toml
echo "Building and signing components with original Cargo.toml and supply-chain folder..."
build_and_sign_components "safe"

# Step 3: Switch to unvettable supply-chain
echo "Switching to unvettable supply-chain..."
for component in command rpn; do
  cd $component
  mv supply-chain supply-chain-safe
  mv supply-chain-unvettable supply-chain
  cd ..
done
build_and_sign_components "unvettable"

for component in command rpn; do
  cd $component
  mv supply-chain supply-chain-unvettable
  mv supply-chain-safe supply-chain
  cd ..
done

# Step 4: Switch to VulnerableCargo.toml
for component in command rpn; do
  cd $component
  mv Cargo.toml SafeCargo.toml
  mv VulnerableCargo.toml Cargo.toml
  cargo generate-lockfile
  cd ..
done

# Step 5: Process VulnerableCargo.toml
echo "Building and signing components with VulnerableCargo.toml..."
build_and_sign_components "vulnerable"

for component in command rpn; do
  cd $component
  mv Cargo.toml VulnerableCargo.toml
  mv SafeCargo.toml Cargo.toml
  cd ..
done

# Extra: Publish the safe composed binary to ghcr.io
echo "Do you want to publish the safe composed binary to ghcr.io? (y/n)"
read publish
if [ "$publish" != "y" ]; then
  echo "Skipping publishing..."
else
  echo "Publishing..."
  # Check whether or not the CR_PAT environment variable is set and ask for it if it is not
  if [ -z "$CR_PAT" ]; then
    echo "Please provide a personal access token for GitHub:"
    read CR_PAT
  fi
  
  # Move the public.key into the publisher-key folder, this key can be used to verify the signature of the binary (not the sigstore signature!!)
  mv public.key publisher-key/public.key

  # We need to upload with wkg and sign with cosign to get the sigstore signature and still be able to later pull and execute the binary
  wkg oci push ghcr.io/titusvm/wasm_rpn_calc_test:latest signed_composed_safe.wasm -u TitusVM -p $CR_PAT
  cosign sign ghcr.io/titusvm/wasm_rpn_calc_test: --registry-username=TitusVM --registry-password=$CR_PAT
fi

echo "All operations completed successfully!"