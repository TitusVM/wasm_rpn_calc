# This is a benchmark for testing the compilation times of the binaries using the `wastime` runtime with and without audit checks.
# The build and sign process for the components to discover overhead in the production pipeline

benchmark_build_and_sign_safe() {
    local component="rpn"
    local suffix="safe"
    cd $component
    # Delete the previous build
    rm -rf target

    # Capture the time taken to execute the commands
    start_time=$(date +%s%N)
    
    cargo vet --output-format=json > ${component}_${suffix}.json
    cargo auditable build --target=wasm32-wasip2
    wasmadder add -i target/wasm32-wasip2/debug/$component.wasm -d ${component}_${suffix}.json -s .vet-v0 -o target/wasm32-wasip2/debug/${component}.wasm
    SIGNED_WASM="../binaries/signed_${component}_${suffix}.wasm"
    wasmsign2 sign -k ../secret.key -i target/wasm32-wasip2/debug/$component.wasm -o $SIGNED_WASM  > /dev/null

    end_time=$(date +%s%N)
    elapsed_time=$((end_time - start_time))
    
    cd ..
    
    # Return the elapsed time in milliseconds
    echo $((elapsed_time / 1000000))
}

benchmark_build() {
    local component="rpn"
    cd $component
    # Delete the previous build
    rm -rf target

    # Capture the time taken to execute the commands
    start_time=$(date +%s%N)
    
    cargo build --target=wasm32-wasip2

    end_time=$(date +%s%N)
    elapsed_time=$((end_time - start_time))
    
    cd ..
    
    # Return the elapsed time in milliseconds
    echo $((elapsed_time / 1000000))
}

echo "Benchmarking build and sign process? (y/n)"
read response
if [ "$response" != "y" ]; then
    echo "Skipping"
else
    echo "Run,Result" > results_safe.csv
    for i in {1..100}
    do
    result=$(benchmark_build_and_sign_safe)
    echo "$i,$result" >> results_safe.csv
    done
fi

echo "Benchmarking normal build process? (y/n)"
read response
if [ "$response" != "y" ]; then
    echo "Skipping"
else
    echo "Benchmarking normal build process..."
    echo "Run,Result" > results_normal.csv
    for i in {1..100}
    do
    result=$(benchmark_build)
    echo "$i,$result" >> results_normal.csv
    done
fi