extern crate gen_cpp;

fn main() {
  let includes = ["\"cpp.h\"".to_string()];
  let sigs = [
    gen_cpp::FunctionSignature::Simple("foo".to_string(), vec!("int".to_string()), None),
    gen_cpp::FunctionSignature::Simple("bar".to_string(), vec!(), Some("int".to_string())),
  ];
  let mut dest = String::new();
  gen_cpp::gen_cpp(&includes, &sigs, &mut dest);
  print!("{}", dest);
}
