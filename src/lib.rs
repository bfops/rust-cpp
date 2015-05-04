use std::borrow::Borrow;

pub type Name = String;
pub type TemplateParam = String;
pub type Param = String;
pub type ReturnType = String;

pub enum FunctionSignature {
  Simple(Name, Vec<Param>, Option<ReturnType>),
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
        dest.push_str(format!("extern \"C\" {} cpp_{}(", ret, name).borrow());
        {
          let mut params = params.iter().enumerate();
          params.next().map(|(i, param)| {
            dest.push_str(format!("{} _{}", param, i).borrow());
            for (i, param) in params {
              dest.push_str(format!(", {} _{}", param, i).borrow());
            }
          });
        }
        dest.push_str(") {\n");
        dest.push_str(format!("  return {}(", name).borrow());
        {
          let len = params.len();
          if len > 0 {
            dest.push_str("_0");
          }
          for i in 1..len {
            dest.push_str(format!(", _{}", i).borrow());
          }
        }
        dest.push_str(");\n");
        dest.push_str("}\n\n");
      },
    }
  }
}
