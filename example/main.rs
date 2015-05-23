#![feature(plugin)]
#![plugin(cpp_bindgen)]

extern crate libc;

#[allow(dead_code)]
mod mycpplib {
  include!(concat!(env!("OUT_DIR"), "/mycpplib.rs"));
}

fn main() {
  cpp!();

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
  println!("");
  println!("[rust] Calling cpp_Foo_int_new_int with 3");
  let p_foo = unsafe {
    mycpplib::cpp_Foo_int_new_int(3)
  };
  println!("");
  println!("[rust] Calling cpp_Foo_int_x_");
  let x_ = unsafe {
    let p_x_ = mycpplib::cpp_Foo_int_x_(p_foo) as *const libc::c_int;
    *p_x_
  };
  println!("[rust] cpp_Foo_x_ returned &{}", x_);
  println!("");
  println!("[rust] Calling cpp_Foo_int_delete");
  unsafe {
    mycpplib::cpp_Foo_int_delete(p_foo);
  }
}
