use std::borrow::Borrow;

use intercalate::intercalate_to;
use types::Binding;

pub fn gen_header(includes: &[String], binds: &[Binding], dest: &mut String) {
  for inc in includes.iter() {
    dest.push_str(format!("#include {}\n", inc).borrow());
  }
  dest.push_str("\n");

  dest.push_str("#ifdef __cplusplus\n");
  dest.push_str("extern \"C\" {\n");
  dest.push_str("#endif\n");
  dest.push_str("\n");

  for bind in binds.iter() {
    match bind {
      &Binding::FreeFunction(ref name, ref ret, ref template_params, ref params) => {
        // function header
        dest.push_str(format!("{} cpp_{}", ret, name).borrow());

        // add template params to the function name
        for param in template_params.iter() {
          dest.push_str("_");
          dest.push_str(param.borrow());
        }

        dest.push_str("(");
        {
          let it = params.iter().map(|s| s.clone());
          intercalate_to(", ", it, dest);
        }
        dest.push_str(");\n");
      },
    }
  }

  dest.push_str("\n");

  dest.push_str("#ifdef __cplusplus\n");
  dest.push_str("}\n");
  dest.push_str("#endif\n");
}
