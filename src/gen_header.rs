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
        dest.push_str("\n");
      },
      &Binding::Struct(ref name, ref template_params, ref fields, ref ctors) => {
        dest.push_str(format!("void* cpp_{}", name).borrow());
        for param in template_params.iter() {
          dest.push_str("_");
          dest.push_str(param.borrow());
        }
        dest.push_str("_new();\n");

        for params in ctors.iter() {
          dest.push_str(format!("void* cpp_{}", name).borrow());
          for param in template_params.iter() {
            dest.push_str("_");
            dest.push_str(param.borrow());
          }
          dest.push_str("_new");
          for param in params.iter() {
            dest.push_str("_");
            dest.push_str(param.borrow());
          }
          dest.push_str("(");
          {
            let it = params.iter().map(|s| s.clone());
            intercalate_to(", ", it, dest);
          }
          dest.push_str(");\n");
        }
        dest.push_str("\n");

        dest.push_str(format!("void cpp_{}", name).borrow());
        for param in template_params.iter() {
          dest.push_str("_");
          dest.push_str(param.borrow());
        }
        dest.push_str(format!("_delete(void* p_{});\n", name).borrow());
        dest.push_str("\n");

        for field in fields.iter() {
          dest.push_str(format!("void* cpp_{}", name).borrow());
          for param in template_params.iter() {
            dest.push_str("_");
            dest.push_str(param.borrow());
          }
          dest.push_str(format!("_{}", field).borrow());
          dest.push_str(format!("(void* p_{});\n", name).borrow());

          dest.push_str(format!("const void* cpp_{}", name).borrow());
          for param in template_params.iter() {
            dest.push_str("_");
            dest.push_str(param.borrow());
          }
          dest.push_str(format!("_{}_const", field).borrow());
          dest.push_str(format!("(const void* p_{});\n", name).borrow());

          dest.push_str("\n");
        }
      },
    }
  }

  dest.push_str("#ifdef __cplusplus\n");
  dest.push_str("}\n");
  dest.push_str("#endif\n");
}
