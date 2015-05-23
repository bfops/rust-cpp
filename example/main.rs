#![feature(plugin)]
#![plugin(cpp_bindgen)]

extern crate libc;

#[allow(dead_code)]
mod mycpplib {
  include!(concat!(env!("OUT_DIR"), "/mycpplib.rs"));
}

fn main() {
  println!("[rust] Calling bar<int>");
  let x = unsafe {
    mycpplib::cpp_bar_int()
  };
  println!("[rust] bar<int> returned {:?}", x);
  println!("");
  println!("[rust] Calling foo");
  let x = unsafe {
    mycpplib::cpp_foo(x)
  };
  println!("[rust] foo returned {:?}", x);
  println!("");
  println!("[rust] Calling new Foo<int> with 3");
  let p_foo = unsafe {
    mycpplib::cpp_Foo_int_new_int(3)
  };
  println!("[rust] new Foo<int> returned {:?}", p_foo);
  println!("");
  println!("[rust] Calling Foo<int>::x_");
  let x_ = unsafe {
    let p_x_ = mycpplib::cpp_Foo_int_x_(p_foo) as *const libc::c_int;
    *p_x_
  };
  println!("[rust] Foo<int>::x_ returned &{}", x_);
  println!("");
  println!("[rust] Calling delete Foo<int>");
  let x = unsafe {
    mycpplib::cpp_Foo_int_delete(p_foo)
  };
  println!("[rust] Foo<int> returned {:?}", x);
}
