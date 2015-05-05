use std::borrow::Borrow;

// There must be a less from-scratch way to do this in Rust..
pub fn intercalate_to<I: Iterator<Item=String>>(sep: &str, mut it: I, dest: &mut String) {
  it.next().map(|s| {
    dest.push_str(s.borrow());
    for s in it {
      dest.push_str(sep);
      dest.push_str(s.borrow());
    }
  });
}
