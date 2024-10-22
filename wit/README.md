# Wasm Interface Type
Defines component model interfaces. The component model was initially made to fix a problem that came with wasm core modules. Core modules can only interface the outside world through a small number of core WebAssembly types (integers and floats).  
The component model provides records and resources with wich complex data structures can be described and the necessary bindings generated.

## From the [`Wasmer` documentation](https://docs.wasmer.io/wasmer-pack/how-to/resources-vs-records):

"The difference between a resource and a record can be subtle when first starting out, but there is a simple rule of thumb that will work 90% of the time:
_Records contain data, resources encapsulate behaviour._"
