# cyclors
Low level RUST APIs for cyclone

## Supported Features

* ```iceoryx```: Enable support for the Iceoryx PSMX plugin in Cyclone DDS (Linux and macOS only).
* ```prefix_symbols```: Prefix the symbols in the Cyclone DDS and Cyclocut libraries with the version of the cyclors crate. This allows for different versions of the crate to be loaded together statically. On macOS and Windows platforms ```llvm-nm``` and ```llvm-objcopy``` are required.

**Note:** The ```iceoryx``` and ```prefix_symbols features are optional and cannot be enabled at the same time.
