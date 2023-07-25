use aramid::{
    CoFn,
    Fiber,
    FiberIterator,
};

mod cofn;

#[test]
fn fiber_iterator_01() {
    let mut fiber = (0..3).into_fiber();
    let f = |x: &_| *x;

    let coro = fiber.next().unwrap();
    assert_eq!(coro.call(f), 0);
    let coro = fiber.next().unwrap();
    assert_eq!(coro.call(f), 1);
    let coro = fiber.next().unwrap();
    assert_eq!(coro.call(f), 2);

    assert!(fiber.next().is_none());
}
