A library to help ease the pain of calling C++ functions from Rust.
It's currently in very early stages, but the eventual ideal is to have a
rustc plugin use this to generate bindings for whatever functions are used
in the Rust code.

This is in contrast to trying to generate an exhaustive `extern "C"` interface
based on a C++ header file, or trying to link directly to C++ by parsing that
header file. The trade off is that this requires generating and compiling C++
code on every build (although there are lots of optimizations to be made).

Example usage is in the `example` directory, and can be run using `cargo run`.
