use minisynth::{eval, synth};

fn assert_synth(spec: &str, templ: &str, consts: &[&str]) {
    println!("specification = {}", spec);
    println!("     template = {}", templ);
    let mut r = synth(spec, templ).unwrap();
    println!("{:#?}", r);

    // Assert that the results make the template fulfill the specification for
    // an arbitrary range of constanst values.
    for x in -255..256 {
        for c in consts {
            r.insert(c.to_string(), x);
        }
        assert_eq!(eval(spec, &r).unwrap(), eval(templ, &r).unwrap());
    }
}

#[test]
fn test_x_mul_ten_is_sum_of_shifts() {
    assert_synth("x * 10", "(x << h1) + (x << h2)", &["x"]);
}

#[test]
fn test_add() {
    assert_synth("3", "h1 + h2", &[]);
}

#[test]
fn test_sub() {
    assert_synth("3", "h1 - h2", &[]);
}

#[test]
fn test_mul() {
    assert_synth("6", "2 * h1", &[]);
}

#[test]
fn test_div() {
    assert_synth("6", "12 / h1", &[]);
}

#[test]
fn test_shr() {
    assert_synth("6", "12 >> h1", &[]);
}

#[test]
fn test_shl() {
    assert_synth("48", "3 << h1", &[]);
}

#[test]
fn test_neg() {
    assert_synth("-42", "-(2 * h1)", &[]);
}

#[test]
fn test_conditional() {
    assert_synth("x * 9", "x << (hb1 ? x : hn1) + (hb2 ? x : hn2)", &["x"]);
}
