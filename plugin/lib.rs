#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;

use syntax::codemap::Span;
use syntax::ast::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, MacEager};
use syntax::ext::build::AstBuilder;
use rustc::plugin::Registry;

fn scape_cpp(cxt: &mut ExtCtxt, span: Span, _: &[TokenTree]) -> Box<MacResult + 'static> {
  syntax::ext::base::DummyResult::any()
}

fn bind_cpp(cxt: &mut ExtCtxt, span: Span, _: &[TokenTree]) -> Box<MacResult + 'static> {
  assert!(false);
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
  let macro_binding;
  match std::env::var("PREBUILD") {
    Ok(val) => {
      macro_binding = scrape_cpp;
    }
    Err(std::env::VarError::NotPresent) => {
      macro_binding = call_cpp;
    },
    e => {
      e.unwrap();
      return
    }
  }

  reg.register_macro("cpp", macro_binding);
}
