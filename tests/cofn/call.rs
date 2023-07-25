use aramid::{
    CoFn,
    Eval,
};

#[test]
fn pass_closure() {
    let cof = Eval::from(1);
    let f = |x: &i32| x * 2;

    assert_eq!(cof.call(f), 2);
    assert_eq!(cof.call(f), 2);
}

#[test]
fn pass_closure_mut_self() {
    let mut cof = Eval::from(1);
    let f = |x: &i32| x * 2;

    assert_eq!(cof.call(f), 2);
    *cof.as_mut() = 2;
    assert_eq!(cof.call(f), 4);
}

#[test]
fn borrow_closure() {
    let cof = Eval::from(1);
    let f = |x: &i32| x * 2;

    assert_eq!(cof.call(&f), 2);
    assert_eq!(cof.call(&f), 2);
}

#[test]
fn mut_closure() {
    let mut fac = 1;

    let mut f = |x: &i32| {
        fac *= 2;
        x * fac
    };

    let cof = Eval::from(1);

    assert_eq!(cof.call(&mut f), 2);
    assert_eq!(cof.call(&mut f), 4);
    assert_eq!(cof.call(f), 8);
}

#[test]
fn mut_closure_mut_self() {
    let mut fac = 1;

    let mut f = |x: &i32| {
        fac *= 2;
        x * fac
    };

    let mut cof = Eval::from(1);

    assert_eq!(cof.call(&mut f), 2);
    assert_eq!(cof.call(&mut f), 4);
    *cof.as_mut() = 0;
    assert_eq!(cof.call(f), 0);
}
