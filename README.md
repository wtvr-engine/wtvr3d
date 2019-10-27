# wtvr3d: a lightweight and modular 3d library written in Rust for WebAssembly

:warning: **wtvr3d is still in very early stages of development, meaning it's not ready to be used yet.**

## Purpose

wtvr3d's purpose is to  offer a WebAssembly alternative to popular JS 3d engines on the web, while ensuring performance and a small overall footprint.

For now most engines that are ported to WebAssembly are just that: *ported* to WebAssembly. Which means that they are generally ill-fitted for use on the web. The idea of ts3d is creating a 3d library that has the mobile web in mind:

 * **Fast load times** thanks to Webassembly's small footprint
 * **Performance** permitted by compile-time optimizations
 * **Modularity** with conditionnal compilation: if you don't need a feature, your users should not have to download and compile it.

## Installing and building

In order to install the library, first install rust and cargo then `wasm-pack`, then clone the repository, then:

    cd wtvr3d;
    wasm-pack build

Enjoy!

## Testing

wtvr3d comes with unit tests built in; once you cloned the project, run them using `cargo test`
