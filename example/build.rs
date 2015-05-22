#![deny(warnings)]

extern crate bindgen;
extern crate rust_cpp;

use std::io::Write;

// TODO: Use `Writer`s instead of `&mut String`s.

fn main() {
  let out_dir = std::env::var("OUT_DIR").unwrap();
  let out_dir = std::path::Path::new(&out_dir);

  // In practice, these directories would be something like
  // /usr/local/lib and /usr/local/include.
  let lib_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
  let lib_dir = std::path::Path::new(&lib_dir);
  let lib_dir = lib_dir.join("mycpplib");
  let include_dir = lib_dir.clone();

  // In practice, the library would already be built/installed.
  build_lib(&lib_dir);

  let sigs = [
    rust_cpp::Binding::FreeFunction(
      "foo".to_string(),
      "void".to_string(),
      vec!(),
      vec!("int".to_string()),
    ),
    rust_cpp::Binding::FreeFunction(
      "bar".to_string(),
      "int".to_string(),
      vec!("int".to_string()),
      vec!(),
    ),
    rust_cpp::Binding::Struct(
      "Foo".to_string(),
      vec!("int".to_string()),
      vec!("x_".to_string()),
      vec!(vec!("int".to_string())),
    ),
  ];

  generate_header(&out_dir, &sigs);
  generate_cpp(&out_dir, &include_dir, &sigs);

  compile_cpp(&out_dir);
  make_shared_lib(&lib_dir, &out_dir);

  generate_rs(&out_dir);

  println!("cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());
  println!("cargo:rustc-link-lib=static=mycpplib");
  println!("cargo:rustc-link-lib=static=stdc++");
}

fn build_lib(lib_dir: &std::path::Path) {
  let mut cmd = std::process::Command::new("make");
  cmd.current_dir(lib_dir);
  let result = cmd.spawn().unwrap().wait().unwrap();
  assert!(result.success());
}

fn generate_header(out_dir: &std::path::Path, sigs: &[rust_cpp::Binding]) {
  let mut dest = String::new();
  rust_cpp::gen_header(&[], sigs, &mut dest);

  let h_path = std::path::Path::new(&out_dir).join("mycpplib_c.h");
  let mut f = std::fs::File::create(h_path).unwrap();
  f.write_all(dest.as_bytes()).unwrap();
}

fn generate_cpp(out_dir: &std::path::Path, include_dir: &std::path::Path, sigs: &[rust_cpp::Binding]) {
  let lib_h_path = include_dir.join("mycpplib.h");
  let lib_h_path = format!("\"{}\"", lib_h_path.to_str().unwrap());
  let c_h_path = out_dir.join("mycpplib_c.h");
  let c_h_path = format!("\"{}\"", c_h_path.to_str().unwrap());
  let includes = [c_h_path, lib_h_path];

  let mut dest = String::new();
  rust_cpp::gen_cpp(&includes, sigs, &mut dest);

  let cpp_path = std::path::Path::new(&out_dir).join("mycpplib_c.cpp");
  let mut f = std::fs::File::create(cpp_path).unwrap();
  f.write_all(dest.as_bytes()).unwrap();
}

fn generate_rs(out_dir: &std::path::Path) {
  let rs_path = std::path::Path::new(&out_dir).join("mycpplib.rs");

  let mut bindings = bindgen::builder();
  bindings.forbid_unknown_types();

  let h_path = std::path::Path::new(&out_dir).join("mycpplib_c.h");
  let h_path = String::from(h_path.to_str().unwrap());
  bindings.header(h_path);

  let bindings = bindings.generate();
  let bindings = bindings.unwrap();
  bindings.write_to_file(rs_path).unwrap();
}

fn compile_cpp(out_dir: &std::path::Path) {
  let mut cmd = std::process::Command::new("g++");
  cmd.current_dir(out_dir);
  cmd.args(&["-c", "mycpplib_c.cpp", "-fPIC"]);
  let result = cmd.spawn().unwrap().wait().unwrap();
  assert!(result.success());
}

// TODO: Link to the C++ and C versions separately.
fn make_shared_lib(lib_dir: &std::path::Path, out_dir: &std::path::Path) {
  let cpp_lib_path = lib_dir.join("libmycpplib.a");
  let mut cmd = std::process::Command::new("cp");
  cmd.args(&[cpp_lib_path.to_str().unwrap(), out_dir.to_str().unwrap()]);
  let result = cmd.spawn().unwrap().wait().unwrap();
  assert!(result.success());

  let mut cmd = std::process::Command::new("ar");
  cmd.current_dir(out_dir);
  cmd.args(&["rvs", "libmycpplib.a", "mycpplib_c.o"]);
  let result = cmd.spawn().unwrap().wait().unwrap();
  assert!(result.success());
}
