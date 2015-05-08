A library to help ease the pain of calling C++ functions from Rust.
It's currently in very early stages, but the eventual ideal is to have a
rustc plugin use this to generate bindings for whatever functions are used
in the Rust code.

**Update: Since [rust-bindgen](https://github.com/crabtw/rust-bindgen) exists, part of this work is redundant. I'll be taking a stab at the remaining parts, but not necessarily in this repo! I'll try to remember to update this page as things develop.**

This is in contrast to trying to generate an exhaustive `extern "C"` interface
based on a C++ header file, or trying to link directly to C++ by parsing that
header file. The trade off is that this requires generating and compiling C++
code on every build (although there are lots of optimizations to be made).

Example usage is in the `example` directory, and can be run using `cargo run`.
