# Static array helpers
## About
This is just for fun project to create rust `static_array` macro to simplify the creation of static arrays.
More specifically it uses internal rust API to automatically derive static array type. 
So it is unstable like [nitrogen triiodide](https://en.wikipedia.org/wiki/Nitrogen_triiodide) on 3rd February 2020.

## Requirements
- Rust 1.42.0-nightly
- I had to specify `LIBRARY_PATH=C:\Windows\System32` to fix `ld: cannot find -lcfgmgr32` in Windows 
- Install rustc-dev component: `rustup toolchain install <toolchain> --component rustc-dev`)

## Usage example
Check [main.rs](examples/static_array/src/main.rs)

## Limitations
It only works for types that are known to the compiler by default (i32, str, Option etc).