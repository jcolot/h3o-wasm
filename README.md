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

# Available methods and properties

The goal is to provide equivalent to all H3 functions 

h3o
- [] .UNITS
- [] .h3IndexToSplitLong(h3Index) ⇒ SplitLong
- [] .splitLongToH3Index(lower, upper) ⇒ H3Index
- [] .isValidCell(h3Index) ⇒ boolean
- [x] .isPentagon(h3Index) ⇒ boolean
- [] .isResClassIII(h3Index) ⇒ boolean
- [x] .getBaseCellNumber(h3Index) ⇒ number
- [] .getIcosahedronFaces(h3Index) ⇒ Array.<number>
- [x] .getResolution(h3Index) ⇒ number
- [x] .latLngToCell(lat, lng, res) ⇒ H3Index
- [x] .cellToLatLng(h3Index) ⇒ CoordPair
- [] .cellToBoundary(h3Index, [formatAsGeoJson]) ⇒ Array.<CoordPair>
- [x] .cellToParent(h3Index, res) ⇒ H3Index
- [] .cellToChildren(h3Index, res) ⇒ Array.<H3Index>
- [] .cellToChildrenSize(h3Index, res) ⇒ number
- [] .cellToCenterChild(h3Index, res) ⇒ H3Index
- [] .cellToChildPos(h3Index, parentRes) ⇒ number
- [] .childPosToCell(childPos, h3Index, childRes) ⇒ H3Index
- [] .gridDisk(h3Index, ringSize) ⇒ Array.<H3Index>
- [] .gridDiskDistances(h3Index, ringSize) ⇒ Array.<Array.<H3Index>>
- [] .gridRingUnsafe(h3Index, ringSize) ⇒ Array.<H3Index>
- [x] .polygonToCells(coordinates, res, [isGeoJson]) ⇒ Array.<H3Index>
- [] .cellsToMultiPolygon(h3Indexes, [formatAsGeoJson]) ⇒ Array.<Array.<Array.<CoordPair>>>
- [] .compactCells(h3Set) ⇒ Array.<H3Index>
- [] .uncompactCells(compactedSet, res) ⇒ Array.<H3Index>
- [] .areNeighborCells(origin, destination) ⇒ boolean
- [] .cellsToDirectedEdge(origin, destination) ⇒ H3Index
- [] .getDirectedEdgeOrigin(edgeIndex) ⇒ H3Index
- [] .getDirectedEdgeDestination(edgeIndex) ⇒ H3Index
- [] .isValidDirectedEdge(edgeIndex) ⇒ boolean
- [] .directedEdgeToCells(edgeIndex) ⇒ Array.<H3Index>
- [] .originToDirectedEdges(h3Index) ⇒ Array.<H3Index>
- [] .directedEdgeToBoundary(edgeIndex, [formatAsGeoJson]) ⇒ Array.<CoordPair>
- [] .gridDistance(origin, destination) ⇒ number
- [] .gridPathCells(origin, destination) ⇒ Array.<H3Index>
- [] .cellToLocalIj(origin, destination) ⇒ CoordIJ
- [] .localIjToCell(origin, coords) ⇒ H3Index
- [] .greatCircleDistance(latLng1, latLng2, unit) ⇒ number
- [x] .cellArea(h3Index, unit) ⇒ number
- [x] .edgeLength(edge, unit) ⇒ number
- [] .getHexagonAreaAvg(res, unit) ⇒ number
- [] .getHexagonEdgeLengthAvg(res, unit) ⇒ number
- [] .cellToVertex(h3Index, vertexNum) ⇒ H3Index
- [] .cellToVertexes(h3Index) ⇒ Array.<H3Index>
- [] .vertexToLatLng(h3Index) ⇒ CoordPair
- [] .isValidVertex(h3Index) ⇒ boolean
- [] .getNumCells(res) ⇒ number
- [] .getRes0Cells() ⇒ Array.<H3Index>
- [] .getPentagons(res) ⇒ Array.<H3Index>
- [] .degsToRads(deg) ⇒ number
- [] .radsToDegs(rad) ⇒ number
- [] .H3Index : string
- [] .H3IndexInput : string | Array.<number>
- [] .CoordIJ
- [] .H3Error
- [] .CoordPair : Array.<number>
- [] .SplitLong : Array.<number>

## Installation

The `h3o-wasm` package is distributed via npm.

```bash
npm install h3o-wasm
```

## Building

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install wasm-bindgen-cli`
* run `wasm-pack build --target web`

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
