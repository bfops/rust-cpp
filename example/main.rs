mod mycpp {
  extern {
    pub fn cpp_foo(_0: i32);
    pub fn cpp_bar_int() -> i32;
  }
}

fn main() {
  println!("Calling cpp_bar_int");
  let x = unsafe {
    mycpp::cpp_bar_int()
  };
  println!("cpp_bar_int returned {}", x);
  println!("");
  println!("Calling cpp_foo");
  unsafe {
    mycpp::cpp_foo(x);
  }
}
