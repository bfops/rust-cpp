extern crate rust_cpp;

use std::io::Write;

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

  generate_cpp(&include_dir, &out_dir);
  generate_rs(&out_dir);
  compile_cpp(&out_dir);
  make_shared_lib(&lib_dir, &out_dir);

  println!("cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());
  println!("cargo:rustc-link-lib=static=mycpplib");
}

fn build_lib(lib_dir: &std::path::Path) {
  let mut cmd = std::process::Command::new("make");
  cmd.current_dir(lib_dir);
  cmd.spawn().unwrap().wait().unwrap();
}

fn generate_cpp(include_dir: &std::path::Path, out_dir: &std::path::Path) {
  let h_path = include_dir.join("mycpplib.h");
  let h_path = format!("\"{}\"", h_path.to_str().unwrap());

  // This section specifies what bindings will be generated.
  let includes = [h_path];
  let sigs = [
    rust_cpp::FunctionSignature::Simple(
      "foo".to_string(),
      "void".to_string(),
      vec!(),
      vec!("int".to_string()),
    ),
    rust_cpp::FunctionSignature::Simple(
      "bar".to_string(),
      "int".to_string(),
      vec!("int".to_string()),
      vec!(),
    ),
  ];

  let mut dest = String::new();
  rust_cpp::gen_cpp(&includes, &sigs, &mut dest);

  let cpp_path = std::path::Path::new(&out_dir).join("mycpplib_c.cpp");
  let mut f = std::fs::File::create(cpp_path).unwrap();
  f.write_all(dest.as_bytes()).unwrap();
}

fn generate_rs(out_dir: &std::path::Path) {
  let sigs = [
    rust_cpp::FunctionSignature::Simple(
      "foo".to_string(),
      "()".to_string(),
      vec!(),
      vec!("i32".to_string()),
    ),
    rust_cpp::FunctionSignature::Simple(
      "bar".to_string(),
      "i32".to_string(),
      vec!("int".to_string()),
      vec!(),
    ),
  ];

  let mut dest = String::new();
  rust_cpp::gen_rs(&sigs, &mut dest);

  let rs_path = std::path::Path::new(&out_dir).join("mycpplib.rs");
  let mut f = std::fs::File::create(rs_path).unwrap();
  f.write_all(dest.as_bytes()).unwrap();
}

fn compile_cpp(out_dir: &std::path::Path) {
  let mut cmd = std::process::Command::new("g++");
  cmd.current_dir(out_dir);
  cmd.args(&["-c", "mycpplib_c.cpp", "-fPIC"]);
  cmd.spawn().unwrap().wait().unwrap();
}

fn make_shared_lib(lib_dir: &std::path::Path, out_dir: &std::path::Path) {
  let cpp_lib_path = lib_dir.join("libmycpplib.a");
  let mut cmd = std::process::Command::new("cp");
  cmd.args(&[cpp_lib_path.to_str().unwrap(), out_dir.to_str().unwrap()]);
  cmd.spawn().unwrap().wait().unwrap();

  let mut cmd = std::process::Command::new("ar");
  cmd.current_dir(out_dir);
  cmd.args(&["rvs", "libmycpplib.a", "mycpplib_c.o"]);
  cmd.spawn().unwrap().wait().unwrap();
}
