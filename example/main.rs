mod cpp;

fn main() {
  println!("Calling cpp_bar_int");
  let x = unsafe {
    cpp::cpp_bar_int()
  };
  println!("cpp_bar_int returned {}", x);
  println!("");
  println!("Calling cpp_foo");
  unsafe {
    cpp::cpp_foo(x);
  }
}
