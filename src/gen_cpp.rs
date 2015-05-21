use std::borrow::Borrow;

use intercalate::intercalate_to;
use types::Binding;

fn push_template_params(template_params: &Vec<String>, dest: &mut String) {
  if template_params.len() > 0 {
    dest.push_str("<");
    let it = template_params.iter().map(|x| x.clone());
    intercalate_to(", ", it, dest);
    dest.push_str(">");
  }
}

pub fn gen_cpp(includes: &[String], binds: &[Binding], dest: &mut String) {
  for inc in includes.iter() {
    dest.push_str(format!("#include {}\n", inc).borrow());
  }
  dest.push_str("\n");

  dest.push_str("extern \"C\" {\n");
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
          let it = params.iter().enumerate().map(|(i, param)| {
            format!("{} _{}", param, i)
          });
          intercalate_to(", ", it, dest);
        }
        dest.push_str(") {\n");

        // function definition
        dest.push_str(format!("  return {}", name).borrow());

        push_template_params(&template_params, dest);

        // parameters to the C++ call
        dest.push_str("(");
        {
          let it = (0..params.len()).map(|i| format!("_{}", i));
          intercalate_to(", ", it, dest);
        }
        dest.push_str(");\n");
        dest.push_str("}\n");

        dest.push_str("\n");
      },
      &Binding::Struct(ref name, ref template_params, ref fields, ref ctors) => {
        dest.push_str(format!("void* cpp_{}", name).borrow());
        for param in template_params.iter() {
          dest.push_str("_");
          dest.push_str(param.borrow());
        }
        dest.push_str("_new() {\n");
        dest.push_str(format!("  return new {}", name).borrow());
        push_template_params(&template_params, dest);
        dest.push_str("();\n");
        dest.push_str("}\n");
        dest.push_str("\n");

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
            let it = params.iter().enumerate().map(|(i, s)| format!("{} _{}", s, i));
            intercalate_to(", ", it, dest);
          }
          dest.push_str(") {\n");

          dest.push_str(format!("  return new {}", name).borrow());
          push_template_params(&template_params, dest);
          dest.push_str("(");
          {
            let it = (0..params.len()).map(|i| format!("_{}", i));
            intercalate_to(", ", it, dest);
          }
          dest.push_str(");\n");

          dest.push_str("}\n");
          dest.push_str("\n");
        }

        dest.push_str(format!("void cpp_{}", name).borrow());
        for param in template_params.iter() {
          dest.push_str("_");
          dest.push_str(param.borrow());
        }
        dest.push_str("_delete(void* p) {\n");
        dest.push_str(format!("  delete ({}", name).borrow());
        push_template_params(&template_params, dest);
        dest.push_str("*)p;\n");
        dest.push_str("}\n");
        dest.push_str("\n");

        for field in fields.iter() {
          dest.push_str(format!("void* cpp_{}", name).borrow());
          for param in template_params.iter() {
            dest.push_str("_");
            dest.push_str(param.borrow());
          }
          dest.push_str(format!("_{}", field).borrow());

          dest.push_str(format!("(void* p) {{\n").borrow());
          dest.push_str(format!("  return &(({}", name).borrow());
          push_template_params(&template_params, dest);
          dest.push_str(format!("*)p)->{};\n", field).borrow());
          dest.push_str("}\n");
          dest.push_str("\n");

          dest.push_str(format!("const void* cpp_{}", name).borrow());
          for param in template_params.iter() {
            dest.push_str("_");
            dest.push_str(param.borrow());
          }
          dest.push_str(format!("_{}_const", field).borrow());

          dest.push_str(format!("(const void* p) {{\n").borrow());
          dest.push_str(format!("  return &((const {}", name).borrow());
          push_template_params(&template_params, dest);
          dest.push_str(format!("*)p)->{};\n", field).borrow());
          dest.push_str("}\n");
          dest.push_str("\n");
        }
      },
    }
  }

  dest.push_str("}\n");
}
