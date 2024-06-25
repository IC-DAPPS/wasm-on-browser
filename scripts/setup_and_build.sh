#!/bin/bash

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null
then
    echo "wasm-pack could not be found. Installing..."
    cargo install wasm-pack
else
    echo "wasm-pack is already installed."
fi

# Build the WebAssembly package
echo "Building WebAssembly package..."
wasm-pack build --target web

echo "Setup and build complete!"