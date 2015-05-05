extern crate gen_cpp;

use std::io::Write;

fn main() {
  let out_dir = std::env::var("OUT_DIR").unwrap();
  let out_dir = std::path::Path::new(&out_dir);

  let bindings_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
  let bindings_dir = std::path::Path::new(&bindings_dir);

  generate_cpp(&bindings_dir, &out_dir);
  compile_cpp(&bindings_dir, &out_dir);
  make_shared_lib(&out_dir);

  println!("cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());
}

fn generate_cpp(bindings_dir: &std::path::Path, out_dir: &std::path::Path) {
  let h_path = bindings_dir.join("cpp.h");
  let h_path = format!("\"{}\"", h_path.to_str().unwrap());

  let includes = [h_path];
  let sigs = [
    gen_cpp::FunctionSignature::Simple(
      "foo".to_string(),
      vec!(),
      vec!("int".to_string()),
      None,
    ),
    gen_cpp::FunctionSignature::Simple(
      "bar".to_string(),
      vec!("int".to_string()),
      vec!(),
      Some("int".to_string()),
    ),
  ];
  let mut dest = String::new();
  gen_cpp::gen_cpp(&includes, &sigs, &mut dest);

  let cpp_path = std::path::Path::new(&out_dir).join("rust.cpp");
  let mut f = std::fs::File::create(cpp_path).unwrap();
  f.write_all(dest.as_bytes()).unwrap();
}

fn compile_cpp(bindings_dir: &std::path::Path, out_dir: &std::path::Path) {
  let cpp_path = bindings_dir.join("cpp.cpp");

  let mut cmd = std::process::Command::new("g++");
  cmd.current_dir(out_dir);
  cmd.args(&["-c", "rust.cpp", cpp_path.to_str().unwrap(), "-fPIC"]);
  cmd.spawn().unwrap().wait().unwrap();
}

fn make_shared_lib(out_dir: &std::path::Path) {
  let mut cmd = std::process::Command::new("ar");
  cmd.current_dir(out_dir);
  cmd.args(&["rvs", "libcpp.a", "rust.o", "cpp.o"]);
  cmd.spawn().unwrap().wait().unwrap();
}
