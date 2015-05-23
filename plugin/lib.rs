#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;

use syntax::codemap::Span;
use syntax::ast::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult};
use syntax::ext::build::AstBuilder;
use rustc::plugin::Registry;

fn scrape_cpp(cxt: &mut ExtCtxt, span: Span, _: &[TokenTree]) -> Box<MacResult + 'static> {
  syntax::ext::base::DummyResult::any(span)
}

fn call_cpp(_: &mut ExtCtxt, _: Span, _: &[TokenTree]) -> Box<MacResult + 'static> {
  panic!("call_cpp unimplemented");
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
  match std::env::var("PREBUILD") {
    Ok(_) => {
      reg.register_macro("cpp", scrape_cpp);
    }
    Err(std::env::VarError::NotPresent) => {
      reg.register_macro("cpp", call_cpp);
    },
    e => {
      e.unwrap();
    }
  }
}
