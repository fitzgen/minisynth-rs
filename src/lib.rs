extern crate failure;
extern crate z3;

mod arena;
mod ast;
mod eval;
mod parser;

use failure::format_err;
use std::collections::HashMap;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

pub fn eval(source: &str, env: &HashMap<String, isize>) -> Result<isize> {
    let ctx = &mut ast::Context::default();
    let p = parser::StartParser::new();
    let node = p.parse(ctx, source).map_err(|e| format_err!("{}", e))?;
    eval::eval(ctx, node, &mut |s| {
        env.get(s)
            .cloned()
            .ok_or_else(|| format_err!("undefined variable: {}", s))
    })
}

#[cfg(test)]
mod tests {
    use super::eval;
    use std::collections::HashMap;

    #[test]
    fn test_add() {
        assert_eq!(eval("1 + 2", &Default::default()).unwrap(), 3);
    }

    #[test]
    fn test_sub() {
        assert_eq!(eval("2 - 1", &Default::default()).unwrap(), 1);
    }

    #[test]
    fn test_mul() {
        assert_eq!(eval("2 * 3", &Default::default()).unwrap(), 6);
    }

    #[test]
    fn test_div() {
        assert_eq!(eval("4 / 2", &Default::default()).unwrap(), 2);
    }

    #[test]
    fn test_shr() {
        assert_eq!(eval("2 >> 1", &Default::default()).unwrap(), 1);
    }

    #[test]
    fn test_shl() {
        assert_eq!(eval("2 << 1", &Default::default()).unwrap(), 4);
    }

    #[test]
    fn test_neg() {
        assert_eq!(eval("-2", &Default::default()).unwrap(), -2);
    }

    #[test]
    fn test_conditional() {
        assert_eq!(eval("1 ? 2 : 3", &Default::default()).unwrap(), 2);
    }

    #[test]
    fn test_var() {
        let vars: HashMap<_, _> = [("a".to_string(), 42)].iter().cloned().collect();
        assert_eq!(eval("a", &vars).unwrap(), 42);
    }
}
