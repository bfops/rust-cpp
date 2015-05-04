mod mycpp {
  #[link(name = "cpp", kind = "static")]
  extern {
    pub fn cpp_foo(_0: i32);
    pub fn cpp_bar() -> i32;
  }
}

fn main() {
  println!("Calling cpp_bar");
  let x = unsafe {
    mycpp::cpp_bar()
  };
  println!("cpp_bar returned {}", x);
  println!("Calling cpp_foo");
  unsafe {
    mycpp::cpp_foo(x);
  }
  println!("Done");
}
