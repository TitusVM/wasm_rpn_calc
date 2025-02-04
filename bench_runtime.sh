# This is a benchmark for testing the execution times of the binaries.
# We assume that composition (which is a process in constant time) has already been done.
# We run the binaries in a loop to discover the overhead in execution time.

benchmark_execution_safe() {
    local SIGNED_WASM="signed_composed_safe.wasm"
    # Capture the time taken to execute the commands
    start_time=$(date +%s%N)

    ~/master-work/pr/wasmtime/target/debug/wasmtime run --audit -k public.key signed_composed_safe.wasm >/dev/null 2>&1

    end_time=$(date +%s%N)
    elapsed_time=$((end_time - start_time))
    
    # Return the elapsed time in milliseconds
    echo $((elapsed_time / 1000000))
}

benchmark_execution_unsafe() {
    local SIGNED_WASM="signed_composed_safe.wasm"
    # Capture the time taken to execute the commands
    start_time=$(date +%s%N)

    ~/master-work/pr/wasmtime/target/debug/wasmtime run signed_composed_safe.wasm >/dev/null 2>&1
    
    end_time=$(date +%s%N)
    elapsed_time=$((end_time - start_time))
    
    # Return the elapsed time in milliseconds
    echo $((elapsed_time / 1000000))
}

echo "Benchmarking the execution of the signed_composed_safe.wasm binary safely? (y/n)"
read response
if [ "$response" != "y" ]; then
    echo "Skipping"
else
    echo "Run,Result" > results_audit.csv
    for i in {1..100}
    do
    result=$(benchmark_execution_safe)
    echo "$i,$result" >> results_audit.csv
    done
fi

echo "Benchmarking the execution of the signed_composed_safe.wasm binary unsafely? (y/n)"
read response
if [ "$response" != "y" ]; then
    echo "Skipping"
else
    echo "Run,Result" > results_unsafe.csv
    for i in {1..100}
    do
    result=$(benchmark_execution_unsafe)
    echo "$i,$result" >> results_unsafe.csv
    done
fi

