use super::Result;
use crate::ast::{self, Node};

pub trait AbstractInterpret {
    /// The output type of this interpreter.
    type Output;

    /// Create a constant output value.
    fn constant(&mut self, c: i64) -> Self::Output;

    /// `lhs + rhs`
    fn add(&mut self, lhs: &Self::Output, rhs: &Self::Output) -> Self::Output;

    /// `lhs - rhs`
    fn sub(&mut self, lhs: &Self::Output, rhs: &Self::Output) -> Self::Output;

    /// `lhs * rhs`
    fn mul(&mut self, lhs: &Self::Output, rhs: &Self::Output) -> Self::Output;

    /// `lhs / rhs`. Fails on divide by zero.
    fn div(&mut self, lhs: &Self::Output, rhs: &Self::Output) -> Result<Self::Output>;

    /// `lhs >> rhs`
    fn shr(&mut self, lhs: &Self::Output, rhs: &Self::Output) -> Self::Output;

    /// `lhs << rhs`
    fn shl(&mut self, lhs: &Self::Output, rhs: &Self::Output) -> Self::Output;

    /// `-e`
    fn neg(&mut self, e: &Self::Output) -> Self::Output;

    /// Returns `1` if `lhs == rhs`, returns `0` otherwise.
    fn eq(&mut self, lhs: &Self::Output, rhs: &Self::Output) -> Self::Output;

    /// Returns `1` if `lhs != rhs`, returns `0` otherwise.
    fn neq(&mut self, lhs: &Self::Output, rhs: &Self::Output) -> Self::Output;

    /// Perform variable lookup for the identifier `var`.
    fn lookup(&mut self, var: &str) -> Result<Self::Output>;
}

pub fn interpret<A>(
    interpreter: &mut A,
    ctx: &mut ast::Context,
    node: ast::NodeId,
) -> Result<A::Output>
where
    A: AbstractInterpret,
{
    match *ctx.node_ref(node) {
        Node::Const(i) => Ok(interpreter.constant(i)),
        Node::Identifier(s) => {
            let s = ctx.interned(s);
            interpreter.lookup(s)
        }
        Node::Addition(lhs, rhs) => {
            let lhs = interpret(interpreter, ctx, lhs)?;
            let rhs = interpret(interpreter, ctx, rhs)?;
            Ok(interpreter.add(&lhs, &rhs))
        }
        Node::Subtraction(lhs, rhs) => {
            let lhs = interpret(interpreter, ctx, lhs)?;
            let rhs = interpret(interpreter, ctx, rhs)?;
            Ok(interpreter.sub(&lhs, &rhs))
        }
        Node::Multiplication(lhs, rhs) => {
            let lhs = interpret(interpreter, ctx, lhs)?;
            let rhs = interpret(interpreter, ctx, rhs)?;
            Ok(interpreter.mul(&lhs, &rhs))
        }
        Node::Division(lhs, rhs) => {
            let lhs = interpret(interpreter, ctx, lhs)?;
            let rhs = interpret(interpreter, ctx, rhs)?;
            interpreter.div(&lhs, &rhs)
        }
        Node::RightShift(lhs, rhs) => {
            let lhs = interpret(interpreter, ctx, lhs)?;
            let rhs = interpret(interpreter, ctx, rhs)?;
            Ok(interpreter.shr(&lhs, &rhs))
        }
        Node::LeftShift(lhs, rhs) => {
            let lhs = interpret(interpreter, ctx, lhs)?;
            let rhs = interpret(interpreter, ctx, rhs)?;
            Ok(interpreter.shl(&lhs, &rhs))
        }
        Node::Negation(e) => {
            let e = interpret(interpreter, ctx, e)?;
            Ok(interpreter.neg(&e))
        }
        Node::Conditional(condition, consequent, alternative) => {
            let condition = interpret(interpreter, ctx, condition)?;
            let consequent = interpret(interpreter, ctx, consequent)?;
            let alternative = interpret(interpreter, ctx, alternative)?;

            let zero = interpreter.constant(0);
            let neq_zero = interpreter.neq(&condition, &zero);
            let eq_zero = interpreter.eq(&condition, &zero);

            let consequent = interpreter.mul(&neq_zero, &consequent);
            let alternative = interpreter.mul(&eq_zero, &alternative);

            Ok(interpreter.add(&consequent, &alternative))
        }
    }
}
