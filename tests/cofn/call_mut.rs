use aramid::CoFnMut;

use super::EvaluateAt;

#[test]
fn pass_closure() {
    let mut cof = EvaluateAt(1);
    let f = |x: &mut i32| *x * 2;

    assert_eq!(cof.call_mut(f), 2);
    assert_eq!(cof.call_mut(f), 2);
}

#[test]
fn pass_closure_mut_self() {
    let mut cof = EvaluateAt(1);
    let f = |x: &mut i32| *x * 2;

    assert_eq!(cof.call_mut(f), 2);
    cof.0 = 2;
    assert_eq!(cof.call_mut(f), 4);
}

#[test]
fn borrow_closure() {
    let mut cof = EvaluateAt(1);
    let f = |x: &mut i32| *x * 2;

    assert_eq!(cof.call_mut(&f), 2);
    assert_eq!(cof.call_mut(&f), 2);
}

#[test]
fn mut_closure() {
    let mut fac = 1;

    let mut f = |x: &mut i32| {
        fac *= 2;
        *x * fac
    };

    let mut cof = EvaluateAt(1);

    assert_eq!(cof.call_mut(&mut f), 2);
    assert_eq!(cof.call_mut(&mut f), 4);
    assert_eq!(cof.call_mut(f), 8);
}

#[test]
fn mut_closure_mut_self() {
    let mut fac = 1;

    let mut f = |x: &mut i32| {
        fac *= 2;
        *x * fac
    };

    let mut cof = EvaluateAt(1);

    assert_eq!(cof.call_mut(&mut f), 2);
    assert_eq!(cof.call_mut(&mut f), 4);
    cof.0 = 0;
    assert_eq!(cof.call_mut(f), 0);
}

#[test]
fn modify_self_via_closure() {
    let f = |x: &mut i32| *x = 0;
    let mut cof = EvaluateAt(1);
    cof.call_mut(f);
    assert_eq!(cof.0, 0);
}
