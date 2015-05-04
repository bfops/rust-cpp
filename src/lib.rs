use std::borrow::Borrow;

pub type Name = String;
pub type TemplateParam = String;
pub type Param = String;
pub type ReturnType = String;

pub enum FunctionSignature {
  Simple(Name, Vec<Param>, Option<ReturnType>),
}

// There must be a less from-scratch way to do this in Rust..
fn intercalate_to<I: Iterator<Item=String>>(sep: &str, mut it: I, dest: &mut String) {
  it.next().map(|s| {
    dest.push_str(s.borrow());
    for s in it {
      dest.push_str(sep);
      dest.push_str(s.borrow());
    }
  });
}

pub fn gen_cpp(includes: &[String], sigs: &[FunctionSignature], dest: &mut String) {
  for inc in includes.iter() {
    dest.push_str(format!("#include {}\n", inc).borrow());
  }
  dest.push_str("\n");

  for sig in sigs.iter() {
    match sig {
      &FunctionSignature::Simple(ref name, ref params, ref ret) => {
        let ret =
          match ret {
            &None => "void",
            &Some(ref s) => s.borrow(),
          };
        dest.push_str(format!("extern \"C\" {} cpp_{}", ret, name).borrow());
        dest.push_str("(");
        {
          let it = params.iter().enumerate().map(|(i, param)| {
            format!("{} _{}", param, i)
          });
          intercalate_to(", ", it, dest);
        }
        dest.push_str(") {\n");
        dest.push_str(format!("  return {}(", name).borrow());
        {
          let it = (0..params.len()).map(|i| format!("_{}", i));
          intercalate_to(", ", it, dest);
        }
        dest.push_str(");\n");
        dest.push_str("}\n\n");
      },
    }
  }
}
