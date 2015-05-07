use std::borrow::Borrow;

use intercalate::intercalate_to;
use types::Binding;

pub fn gen_rs(binds: &[Binding], dest: &mut String) {
  dest.push_str("extern {\n");

  for bind in binds.iter() {
    match bind {
      &Binding::FreeFunction(ref name, ref ret, ref template_params, ref params) => {
        // function header
        dest.push_str(format!("  pub fn cpp_{}", name).borrow());

        // add template params to the function name
        for param in template_params.iter() {
          dest.push_str("_");
          dest.push_str(param.borrow());
        }

        dest.push_str("(");
        {
          let it = params.iter().enumerate().map(|(i, param)| {
            format!("_{}: {}", i, param)
          });
          intercalate_to(", ", it, dest);
        }
        dest.push_str(format!(") -> {};\n", ret).borrow());
      },
    }
  }

  dest.push_str("}");
}
