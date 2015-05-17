extern crate libc;

mod mycpplib {
  include!(concat!(env!("OUT_DIR"), "/mycpplib.rs"));
}

fn main() {
  println!("[rust] Calling cpp_bar_int");
  let x = unsafe {
    mycpplib::cpp_bar_int()
  };
  println!("[rust] cpp_bar_int returned {:?}", x);
  println!("");
  println!("[rust] Calling cpp_foo");
  let x = unsafe {
    mycpplib::cpp_foo(x)
  };
  println!("[rust] cpp_foo returned {:?}", x);
}
