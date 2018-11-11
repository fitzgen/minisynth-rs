extern crate failure;
extern crate z3;

mod arena;
mod ast;
mod eval;
mod parser;

use failure::format_err;
use std::collections::HashMap;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

pub fn eval(source: &str, env: HashMap<String, isize>) -> Result<isize> {
    let ctx = &mut ast::Context::default();
    let p = parser::StartParser::new();
    let node = p.parse(ctx, source).map_err(|e| format_err!("{}", e))?;
    eval::eval(ctx, node, &mut |s| {
        env.get(s)
            .cloned()
            .ok_or_else(|| format_err!("undefined variable: {}", s))
    })
}
