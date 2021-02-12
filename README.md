# Conway's Game of Life with Rust & WebAssembly

This implementation of Conway's Game of Life is based on this [tutorial](https://rustwasm.github.io/docs/book/game-of-life/introduction.html). All the Rust code that is compiled into WebAssemby is in `./gol_wasm`. It is used in the web app in the `./app` directory. Here are the instruction how to run it on the local system:

## Requirements

* install [Rust](https://www.rust-lang.org/tools/install)
* install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
* install `cargo-generate` with this command: `cargo install cargo-generate`
* install [NodeJs](https://nodejs.org/en/download/package-manager/) if not yet on your system (version 14.x or later)
* install yarn: `npm install -g yarn`

## Build WebAssembly Files

* open terminal and go into `./gol_wasm`
* run `yarn build`

## Run Game of Life App

* open second terminal and go into `./app`
* run `yarn install`
* run `yarn start`
* open URL http://localhost:8080/
