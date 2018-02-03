# ts3D: a lightweight and modular 3d library written in Rust for WebAssembly

:warning: **ts3d is still in very early stages of development, meaning it's not ready to be used yet.**

## Purpose

ts3d's purpose is to  offer a WebAssembly alternative to popular JS 3d engines on the web, while ensuring performance and a small overall footprint.

For now most engines that are ported to WebAssembly are just that: *ported* to WebAssembly. Which means that they are generally ill-fitted for use on the web. The idea of ts3d is creating a 3d library that has the mobile web in mind:

 * **Fast load times** thanks to Webassembly's small footprint
 * **Performance** permitted by compile-time optimizations
 * **Modularity** with conditionnal compilation: if you don't need a feature, your users should not have to download and compile it.

## Uses outside of WebAssembly

Of course, since ts3d is primarily a 3d library destined to be used in a 3d engine, it could be used in any other setup, including as a normal crate for Rust. It's aimed at providing standard 3d math and a scene structure, but still needs a rendering backend. For ts-engine, this will be handled by JS with WebGL.

## Installing and building

In order to install the library, first install rust and cargo, then clone the repository, then:

    cd ts3d;
    cargo build

this will build the library for your default platform. If you want to build ts3d for WebAssembly, here's what you should do instead, since at the time of writing, the rust wasm target has not yet made its way to stable:

 * Update rustup : `rustup update;`
 * Install the wasm target : `rustup target add wasm32-unknown-unknown --toolchain nightly`
 * Install [wasm-gc](https://github.com/alexcrichton/wasm-gc) : `cargo install --git https://github.com/alexcrichton/wasm-gc`
 * `cd` into ts3d's directory : `cd ts3d`
 * Build the library for the wasm target : `cargo  +nightly build --target wasm32-unknown-unknown --release`
 * Run `wasm-gc` on the output to strip it of any unused sections that rust might have included

Enjoy!

## Testing

ts3d comes with unit tests built in; once you cloned the project, run them using `cargo test`
