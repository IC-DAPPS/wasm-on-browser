# WebAssembly Rust Project

This project demonstrates how to run WebAssembly (Wasm) compiled from Rust in a browser.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/) (for running a local server)

## for more detail read this

- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)

## Setup and Build

1. Clone this repository:
   ```
   git clone <your-repository-url>
   cd <your-project-directory>
   ```

2. Run the setup and build script:
   ```
   chmod +x setup_and_build.sh
   ./setup_and_build.sh
   ```
   This script will install wasm-pack if it's not already installed and then build your WebAssembly package.

## Running the Project

1. Start a local server. You can use Python's built-in HTTP server:
   ```
   python3 -m http.server 8080
   ```
   Or if you're using Python 2:
   ```
   python -m SimpleHTTPServer 8080
   ```

2. Open your web browser and navigate to:
   ```
   http://localhost:8080
   ```

3. You should see a button labeled "Run Main Function". Click it to execute the WebAssembly function.

4. Open your browser's developer tools (usually F12) and check the console tab to see the output.

## Project Structure

- `src/lib.rs`: Contains the Rust code that will be compiled to WebAssembly.
- `index.html`: The HTML file that loads and runs the WebAssembly module.
- `pkg/`: Directory created by wasm-pack containing the compiled WebAssembly and JavaScript glue code.

## Modifying the Project

1. Edit the Rust code in `src/lib.rs`.
2. Rebuild the project using:
   ```
   wasm-pack build --target web
   ```
3. Refresh your browser to see the changes.

## Troubleshooting

- If you encounter any CORS errors, make sure you're using a local server and not opening the HTML file directly.
- Check that all files are in the correct locations, especially the generated files in the `pkg/` directory.
- Ensure your browser supports WebAssembly.

## Learn More

- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/)
- [wasm-pack documentation](https://rustwasm.github.io/wasm-pack/book/)

```

This README provides a comprehensive guide for setting up, building, and running your WebAssembly project. It includes:

1. Prerequisites
2. Setup and build instructions
3. How to run the project
4. Project structure explanation
5. Instructions for modifying the project
6. Troubleshooting tips
7. Links for further learning

You can customize this README further based on your specific project needs or any additional features you might add in the future.