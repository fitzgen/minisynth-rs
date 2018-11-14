use minisynth::eval;
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
