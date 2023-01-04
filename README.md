# Welcome to my portfolio repository

Disclaimer: I am not really a UI designer, and this is definietly a WIP. Some parts of the code are a mess.

This repository contains the code for the webserver and app at [nfreesto.com](http://nfreesto.com) and contains one rust module for each.

## server

The webserver is very basic because it doesn't need to do any fancy post requests or handle any dynamic elements (yet).

It is written in Rust using [Warp](https://github.com/seanmonstar/warp). Warp uses [Tokio](https://github.com/tokio-rs/tokio) for an asyncronus runtime.

Currently, on run, it first compiles the app then serves it to localhost, by default.

## app

The app is written in Rust and compiled to webassembly using [yew](https://github.com/yewstack/yew), [yew-router](https://github.com/yewstack/yew) for the actual app, and naturally [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) and [web-sys](https://github.com/rustwasm/wasm-bindgen/tree/master/crates/web-sys) for web bindings.