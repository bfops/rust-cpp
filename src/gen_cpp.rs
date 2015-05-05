use std::borrow::Borrow;

use intercalate::intercalate_to;
use types::FunctionSignature;

pub fn gen_cpp(includes: &[String], sigs: &[FunctionSignature], dest: &mut String) {
  for inc in includes.iter() {
    dest.push_str(format!("#include {}\n", inc).borrow());
  }
  dest.push_str("\n");

  for sig in sigs.iter() {
    match sig {
      &FunctionSignature::Simple(ref name, ref ret, ref template_params, ref params) => {
        // function header
        dest.push_str(format!("extern \"C\" {} cpp_{}", ret, name).borrow());

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

        // template parameters to the C++ call
        if template_params.len() > 0 {
          dest.push_str("<");
          let it = template_params.iter().map(|x| x.clone());
          intercalate_to(", ", it, dest);
          dest.push_str(">");
        }

        // parameters to the C++ call
        dest.push_str("(");
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
