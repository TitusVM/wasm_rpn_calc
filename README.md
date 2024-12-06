# WASM Component demo
This implementation of a calculator in [Reverse Polish Notation (RPN)](https://en.wikipedia.org/wiki/Reverse_Polish_notation) serves to demonstrate the functionnality of the [Wasm component model](https://component-model.bytecodealliance.org/).

## Long running main
Checkout the `long_running` branch to build an executable that takes a long time to run.

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






