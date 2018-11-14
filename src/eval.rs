use super::Result;
use crate::abstract_interpret::{interpret, AbstractInterpret};
use crate::ast::{self, NodeId};
use failure::{bail, format_err};
use std::collections::HashMap;

struct Eval<'a> {
    env: &'a HashMap<String, i64>,
}

impl<'a> AbstractInterpret for Eval<'a> {
    type Output = i64;

    fn constant(&mut self, c: i64) -> i64 {
        c
    }

    fn add(&mut self, lhs: &i64, rhs: &i64) -> i64 {
        lhs + rhs
    }

    fn sub(&mut self, lhs: &i64, rhs: &i64) -> i64 {
        lhs - rhs
    }

    fn mul(&mut self, lhs: &i64, rhs: &i64) -> i64 {
        lhs * rhs
    }

    fn div(&mut self, lhs: &i64, rhs: &i64) -> Result<i64> {
        if *rhs == 0 {
            bail!("divide by zero");
        }
        Ok(lhs / rhs)
    }

    fn shr(&mut self, lhs: &i64, rhs: &i64) -> i64 {
        lhs >> rhs
    }

    fn shl(&mut self, lhs: &i64, rhs: &i64) -> i64 {
        lhs << rhs
    }

    fn neg(&mut self, e: &i64) -> i64 {
        -e
    }

    fn eq(&mut self, lhs: &i64, rhs: &i64) -> i64 {
        (lhs == rhs) as i64
    }

    fn neq(&mut self, lhs: &i64, rhs: &i64) -> i64 {
        (lhs != rhs) as i64
    }

    fn lookup(&mut self, var: &str) -> Result<i64> {
        self.env
            .get(var)
            .cloned()
            .ok_or_else(|| format_err!("undefined variable: {}", var))
    }
}

pub fn eval(ctx: &mut ast::Context, node: NodeId, env: &HashMap<String, i64>) -> Result<i64> {
    let eval = &mut Eval { env };
    interpret(eval, ctx, node)
}
