extern crate failure;
extern crate z3;

mod abstract_interpret;
mod ast;
mod eval;
mod parser;
mod synthesize;

use failure::format_err;
use std::collections::HashMap;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

pub fn eval(source: &str, env: &HashMap<String, i64>) -> Result<i64> {
    let ctx = &mut ast::Context::default();
    let p = parser::StartParser::new();
    let node = p.parse(ctx, source).map_err(|e| format_err!("{}", e))?;
    eval::eval(ctx, node, env)
}

pub fn synth(specification: &str, template: &str) -> Result<HashMap<String, i64>> {
    let config = z3::Config::new();
    let z3_ctx = &z3::Context::new(&config);

    let ast_ctx = &mut ast::Context::default();
    let p = parser::StartParser::new();

    let specification = p
        .parse(ast_ctx, specification)
        .map_err(|e| format_err!("error parsing specification:{}", e))?;
    let template = p
        .parse(ast_ctx, template)
        .map_err(|e| format_err!("error parsing template: {}", e))?;

    synthesize::synthesize(z3_ctx, ast_ctx, specification, template)
}
