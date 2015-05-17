A library to help ease the pain of calling C++ functions from Rust.
The basic idea is to generate a C-compatible library from a list of
C++ functions and structs to wrap, and then to use
[rust-bindgen](https://github.com/crabtw/rust-bindgen) to generate the Rust
interface.

This is in contrast to trying to generate an exhaustive `extern "C"` interface
based on a C++ header file, or trying to link directly to C++ by parsing a
header file. The trade off is that this requires generating and compiling C++
code on every build (although there are lots of optimizations to be made).

It's currently in very early stages, but the eventual ideal is to have a rustc
plugin generate the list of C++ calls by scraping the calls made from the Rust
code.

Example usage is in the `example` directory, and can be run using `cargo run`.
