# WASM Component demo
This implementation of a calculator in [Reverse Polish Notation (RPN)](https://en.wikipedia.org/wiki/Reverse_Polish_notation) serves to demonstrate the functionnality of the [Wasm component model](https://component-model.bytecodealliance.org/).

## Long running main
Checkout the `long_running` branch to build an executable that takes a long time to run.

## Running the demo
There are two essential scripts to run the demo:
- `./setup.sh` which will build all the binaries, sign them and publish them if you choose to do so (publishing requires a [GitHub token](https://docs.github.com/en/github/authenticating-to-github/creating-a-personal-access-token))
- `./run.sh` which will verify the signatures of the components, compose them and run the resulting binary with `wasmtime` (and calling the audit process `--audit`)

The scripts require following tools:
- [`cargo-vet`](https://crates.io/crates/cargo-vet) 0.10.0 for generating vet information of the source code
- [`cargo-auditable`](https://crates.io/crates/cargo-auditable) 1.84.0-nightly for building the auditable binaries
- [`wasmadder-cli`](https://crates.io/crates/wasmadder) 0.1.0 for adding vet info to the wasm binaries
- [`wasmsign2-cli`](https://crates.io/crates/wasmsign2-cli) 0.2.6 for signing the wasm binaries
- [`wac-cli`](https://crates.io/crates/wac-cli) 0.6.0 for composing the wasm binaries
- [`wkg`](https://crates.io/crates/wkg) 0.9.0 for publishing the wasm binaries
- [`cosign`](https://github.com/sigstore/cosign) 2.4.1 for signing and verifying published packages
- my custom `wasmtime` fork: [wasmtime](https://github.com/TitusVM/wasmtime) for running packages

### `setup.sh`

The `setup.sh` script automates the building and signing process of the project's components using both safe and vulnerable versions of dependencies. It performs the following steps:

1. **Generate Signing Keys**: Creates public and secret keys for signing the WebAssembly components.
2. **Build and Sign with Safe Dependencies**: Builds and signs the components using the original `Cargo.toml` with vetted dependencies.
3. **Switch to Unvetted Supply Chain**: Replaces the supply chain folders to use unvetted dependencies and rebuilds and signs the components.
4. **Switch to Vulnerable Dependencies**: Replaces the `Cargo.toml` with `VulnerableCargo.toml` to build and sign the components with vulnerable dependencies.
5. **Restore Original State**: Restores the original `Cargo.toml` and supply chain folders to their safe versions.
6. **Optional Publishing**: Offers the option to publish the safe composed binary to GitHub Container Registry (`ghcr.io`), signing it with Cosign for verification.

### `run.sh`
The `run.sh` script automates the verification and execution of the composed binary using the following steps:

1. **Verify the package's signature**: Verifies the signatures of the components using `cosign`.
2. **Download the package**: Downloads the package from the GitHub Container Registry (`ghcr.io`).
3. **Runs the package**: Runs the package using `wasmtime` and the `--audit` flag to perform the audit process.

## Structure
The example is divided into two seperate components: 
- The `rpn` library responsible for implementing the calculator logic
- The `command` binary which uses the library to do some math (similar to the `main` function of a normal program)

To be able to execute the example in a WebAssembly runtime such as [`wasmtime`](https://github.com/bytecodealliance/wasmtime), we first need to generate the [component bindings](https://github.com/bytecodealliance/wit-bindgen), build the component binaries and then **[compose](https://component-model.bytecodealliance.org/creating-and-consuming/composing.html)** them.

### `bindings.rs` and building
The generated `bindings.rs` files must be kept up-to-date with the `wit` specifications in `wit/*/world.wit` and will be updated with the [cargo-component](https://github.com/bytecodealliance/cargo-component) build command:
```
cargo component build --target=<TARGET>
```
Where `TARGET` is something wasmy from [here](https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1.html) that supports the component model, I recommend `wasm32-wasip2` but that requires rustc nightly.
FYI: this command must be run in each subfolder that contains component source code i.e. `./command/` and `./rpn`.

## The composition step
The components that are generated (in `command or rpn` + `/target/<TARGET>/debug/` must now be composed using [`wac`](https://github.com/bytecodealliance/wac):
```
wac plug command/target/<TARGET>/debug/command.wasm --plug rpn/target/<TARGET>/debug/rpn.wasm -o composed.wasm
```
This step is crucial as it [`plugs`](https://github.com/bytecodealliance/wac) the library into the sockets of the command program. This means that the exports of the library are bound to the imports of the `command` component. The resulting `composed.wasm` binary does not export `rpn`'s exports but only those specifed by `command`.

## Run with your favorite runtime
E.g. [`wasmtime`](https://github.com/bytecodealliance/wasmtime)
```
$ wasmtime run composed.wasm
3
```
The subsequent output should be the result of the math that we did in the `command` component using the `rpn` library.






