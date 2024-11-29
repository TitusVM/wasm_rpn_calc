#!/bin/bash

# Exit immediately if any command fails
set -e

# Step 0: Generate signing keys
echo "Generating signing keys..."
wasmsign2 keygen --public-key public.key --secret-key secret.key

# Create the binaries folder if it doesn't exist
mkdir -p binaries

# Function to build, sign, and compose components
build_and_sign_components() {
  local suffix=$1  # Suffix for signed binaries and composed WASM
  
  # Build and sign components
  for component in command rpn; do
    echo "Processing component: $component ($suffix)"
    
    # Change to the component directory
    cd $component
    
    # Compile the component
    echo "Building component..."
    cargo auditable build --target=wasm32-wasip2
    
    # Sign the component
    echo "Signing component..."
    SIGNED_WASM="../binaries/signed_${component}_${suffix}.wasm"
    wasmsign2 sign -k ../secret.key -i target/wasm32-wasip2/debug/$component.wasm -o $SIGNED_WASM
    
    # Return to the root directory
    cd ..
  done

  # Compose the components
  echo "Composing components ($suffix)..."
  COMPOSED_WASM="binaries/composed_${suffix}.wasm"
  wac plug binaries/signed_command_${suffix}.wasm --plug binaries/signed_rpn_${suffix}.wasm -o $COMPOSED_WASM
  
  # Sign the composed component
  SIGNED_COMPOSED_WASM="signed_composed_${suffix}.wasm"
  echo "Signing composed component ($suffix)..."
  wasmsign2 sign -k secret.key -i $COMPOSED_WASM -o $SIGNED_COMPOSED_WASM
}

# Step 1: Process original Cargo.toml
echo "Building and signing components with original Cargo.toml..."
build_and_sign_components "safe"

# Step 2: Switch to VulnerableCargo.toml
for component in command rpn; do
  echo "Switching Cargo.toml for component: $component"
  
  cd $component
  mv Cargo.toml SafeCargo.toml
  mv VulnerableCargo.toml Cargo.toml
  cd ..
done

# Step 3: Process VulnerableCargo.toml
echo "Building and signing components with VulnerableCargo.toml..."
build_and_sign_components "vulnerable"

# Step 4: Restore original Cargo.toml
for component in command rpn; do
  echo "Restoring original Cargo.toml for component: $component"
  
  cd $component
  mv Cargo.toml VulnerableCargo.toml
  mv SafeCargo.toml Cargo.toml
  cd ..
done

echo "All operations completed successfully!"
