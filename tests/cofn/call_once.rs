use aramid::CoFnOnce;

use super::EvaluateAt;

#[test]
fn move_closure() {
    let cof = EvaluateAt(1);
    let f = |x| x * 2;

    assert_eq!(cof.call_once(f), 2);

    // cof moved by value. This won't compile:
    // assert_eq!(cof.call_once(f), 2);
}

#[test]
fn borrow_closure() {
    let cof = EvaluateAt(1);
    let f = |x| x * 2;

    assert_eq!(cof.call_once(&f), 2);

    let cof = EvaluateAt(1);
    assert_eq!(cof.call_once(f), 2);
}

#[test]
fn mut_closure() {
    let mut fac = 1;

    let mut f = |x| {
        fac *= 2;
        x * fac
    };

    let cof = EvaluateAt(1);
    assert_eq!(cof.call_once(&mut f), 2);

    let cof = EvaluateAt(1);
    assert_eq!(cof.call_once(&mut f), 4);

    let cof = EvaluateAt(2);
    assert_eq!(cof.call_once(f), 16);
}
