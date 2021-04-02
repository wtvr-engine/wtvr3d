# wtvr3d: a modular, mobile-first 3d library written in Rust for WebAssembly and WebGL2

:warning: **wtvr3d is still in very early stages of development, meaning it's not ready to be used yet.**

## Purpose

wtvr3d's purpose is to  offer a WebAssembly alternative to popular JS 3d engines on the web, while ensuring performance and a small overall footprint.

It is also aimed at providing the best experience possible to mobile users, by allowing progressive and gracious graphical downgrade on middle-to-low-end devices ( resolution, shader complexity, displayed objects, view distance...)

For now most engines that are ported to WebAssembly are just that: *ported* to WebAssembly. Which means that they are generally ill-fitted for use on the web, and generally not supported on the mobile web. The idea of wtvr3d is creating a 3d library that has the mobile web in mind:

 * **Fast load times** thanks to Webassembly's small footprint
 * **Performance** permitted by compile-time optimizations
 * **Modularity** with conditionnal compilation: if you don't need a feature, your users should not have to download and compile it.
 * **Progressive graphical upgrade or downgrade** with profiles and alternative shaders, textures, resolution, view distance, post-processing...

## Installing and building

In order to install the library, first install rust and cargo then `wasm-pack`, then clone the repository, then:

    cd wtvr3d;
    wasm-pack build

Enjoy!
