use super::Result;
use crate::abstract_interpret::{interpret, AbstractInterpret};
use crate::ast::{self, NodeId};
use failure::{bail, format_err};
use std::collections::HashMap;

struct Eval<'a> {
    env: &'a HashMap<String, isize>,
}

impl<'a> AbstractInterpret for Eval<'a> {
    type Output = isize;

    fn constant(&mut self, c: isize) -> isize {
        c
    }

    fn add(&mut self, lhs: &isize, rhs: &isize) -> isize {
        lhs + rhs
    }

    fn sub(&mut self, lhs: &isize, rhs: &isize) -> isize {
        lhs - rhs
    }

    fn mul(&mut self, lhs: &isize, rhs: &isize) -> isize {
        lhs * rhs
    }

    fn div(&mut self, lhs: &isize, rhs: &isize) -> Result<isize> {
        if *rhs == 0 {
            bail!("divide by zero");
        }
        Ok(lhs / rhs)
    }

    fn shr(&mut self, lhs: &isize, rhs: &isize) -> isize {
        lhs >> rhs
    }

    fn shl(&mut self, lhs: &isize, rhs: &isize) -> isize {
        lhs << rhs
    }

    fn neg(&mut self, e: &isize) -> isize {
        -e
    }

    fn eq(&mut self, lhs: &isize, rhs: &isize) -> isize {
        (lhs == rhs) as isize
    }

    fn neq(&mut self, lhs: &isize, rhs: &isize) -> isize {
        (lhs != rhs) as isize
    }

    fn lookup(&mut self, var: &str) -> Result<isize> {
        self.env
            .get(var)
            .cloned()
            .ok_or_else(|| format_err!("undefined variable: {}", var))
    }
}

pub fn eval(ctx: &mut ast::Context, node: NodeId, env: &HashMap<String, isize>) -> Result<isize> {
    let eval = &mut Eval { env };
    interpret(eval, ctx, node)
}
