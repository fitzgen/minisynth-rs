use super::Result;
use crate::ast::{self, Node};
use failure::bail;

pub fn eval<L>(ctx: &mut ast::Context, node: ast::NodeId, lookup: &mut L) -> Result<isize>
where
    L: for<'a> FnMut(&'a str) -> Result<isize>,
{
    match *ctx.node_ref(node) {
        Node::Const(i) => Ok(i),
        Node::Identifier(s) => {
            let s = ctx.interned(s);
            lookup(s)
        }
        Node::Addition(lhs, rhs) => {
            let lhs = eval(ctx, lhs, lookup)?;
            let rhs = eval(ctx, rhs, lookup)?;
            Ok(lhs + rhs)
        }
        Node::Subtraction(lhs, rhs) => {
            let lhs = eval(ctx, lhs, lookup)?;
            let rhs = eval(ctx, rhs, lookup)?;
            Ok(lhs - rhs)
        }
        Node::Multiplication(lhs, rhs) => {
            let lhs = eval(ctx, lhs, lookup)?;
            let rhs = eval(ctx, rhs, lookup)?;
            Ok(lhs * rhs)
        }
        Node::Division(lhs, rhs) => {
            let lhs = eval(ctx, lhs, lookup)?;
            let rhs = eval(ctx, rhs, lookup)?;
            if rhs == 0 {
                bail!("divide by zero");
            }
            Ok(lhs / rhs)
        }
        Node::RightShift(lhs, rhs) => {
            let lhs = eval(ctx, lhs, lookup)?;
            let rhs = eval(ctx, rhs, lookup)?;
            Ok(lhs >> rhs)
        }
        Node::LeftShift(lhs, rhs) => {
            let lhs = eval(ctx, lhs, lookup)?;
            let rhs = eval(ctx, rhs, lookup)?;
            Ok(lhs << rhs)
        }
        Node::Negation(n) => {
            let n = eval(ctx, n, lookup)?;
            Ok(-n)
        }
        Node::Conditional(condition, consequent, alternative) => {
            let condition = eval(ctx, condition, lookup)?;
            let consequent = eval(ctx, consequent, lookup)?;
            let alternative = eval(ctx, alternative, lookup)?;
            Ok((condition != 0) as isize * consequent + (condition == 0) as isize * alternative)
        }
    }
}
