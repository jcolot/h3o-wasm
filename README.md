# h3o-wasm

[![Crates.io](https://img.shields.io/crates/v/h3o.svg)](https://crates.io/crates/h3o)
[![Docs.rs](https://docs.rs/h3o/badge.svg)](https://docs.rs/h3o)
[![CI Status](https://github.com/HydroniumLabs/h3o/actions/workflows/ci.yml/badge.svg)](https://github.com/HydroniumLabs/h3o/actions)
[![Coverage](https://img.shields.io/codecov/c/github/HydroniumLabs/h3o)](https://app.codecov.io/gh/HydroniumLabs/h3o)
[![License](https://img.shields.io/badge/license-BSD-green)](https://opensource.org/licenses/BSD-3-Clause)

A [Rust](https://rustlang.org) implementation of the [H3](https://h3geo.org) geospatial indexing system, compiled to WebAssembly for use in JavaScript projects.

## Design
**Experimental**: this package compiles some functions from `h3o` to WASM. Performance might be slower than the reference `h3-js` implementation
`h3o` is a Rust reimplementation of the H3 geospatial indexing system, designed with the following goals:
- Leverage Rust's strong typing for safer usage.
- Provide a 100% Rust solution, eliminating C dependencies for painless compilation to WebAssembly and easier link-time optimization.
- Match or exceed the performance of the reference H3 library.

`h3o-wasm` exposes `h3o` functions to Javascript

## Installation

The `h3o-wasm` package is distributed via npm.

```bash
npm install h3o-wasm
```

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install h3o`

## Usage

```javascript
import init, * as h3o from "h3o-wasm";
async function run() {
    await init(); // Initialize the WASM module
    const h3Index = h3o.latLngToCell(37.3615593, -122.0553238, 7);
    console.log("H3 Index: ", h3Index);
}
run();
```

## Why this name?

Rust is an iron oxide.
A Rust version of H3 is an H3 oxide, in other word $H_3O$ (a.k.a hydronium).
Chemically speaking this is wrong ( $H_3O$ is produced by protonation of
$H_2O$, not oxidation of $H_3$), but ¯\\_(ツ)_/¯

## License

[BSD 3-Clause](./LICENSE)
