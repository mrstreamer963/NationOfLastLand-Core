# Example WASM Project

This is a template for a WebAssembly project using Rust and wasm-bindgen.

## Features

- Basic WASM functions that can be called from JavaScript
- Integration with the core crate
- Async function support
- HTML test page with interactive examples
- Build configuration with Makefile targets

## Building

### Prerequisites

Make sure you have `wasm-pack` installed:

```bash
cargo install wasm-pack
```

### Build Commands

From the project root:

```bash
# Build the WASM package
make wasm-example

# Build in release mode
make wasm-example-release
```

## Running

After building, serve the files:

```bash
# Start a local HTTP server
make serve-wasm-example
```

Then open `http://localhost:8000` in your browser.

## Project Structure

- `Cargo.toml` - Package configuration with WASM dependencies
- `src/lib.rs` - Main WASM library code
- `index.html` - Test page with interactive examples
- `pkg/` - Generated WASM package (after build)

## Available Functions

The WASM module exports several functions that can be called from JavaScript:

- `get_core_info()` - Returns information about core integration
- `async_example()` - Demonstrates async functionality

## Development

To modify the WASM code, edit `src/lib.rs` and rebuild with `make wasm-example`.

The HTML test page in `index.html` demonstrates how to use the WASM functions from JavaScript.
