use aramid::{
    Fiber,
    FiberIterator,
    State,
};

struct MockFiber {
    count:  u64,
    steps:  u64,
    result: bool,
}

impl Fiber for MockFiber {
    type Output = bool;
    type Yield = u64;

    fn run(mut self) -> aramid::State<Self> {
        if self.count < self.steps {
            self.count += 1;
            State::Yield(self)
        } else {
            State::Done(self.result)
        }
    }

    fn get(&mut self) -> Option<Self::Yield> {
        Some(self.count)
    }
}

#[test]
fn mock_fiber_into_iter() {
    let mut result = true;
    let coll = MockFiber {
        count:  0,
        steps:  3,
        result: true,
    }
    .into_iter(|x| result = x)
    .collect::<Vec<_>>();

    assert_eq!(coll, &[1, 2, 3]);
    assert!(result);
}

#[test]
fn mock_fiber_into_iter_fused() {
    let mut result = true;
    let mut iter = MockFiber {
        count:  0,
        steps:  3,
        result: true,
    }
    .into_iter(|x| result = x);

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    assert!(result);
}

#[test]
fn fiber_iterator_ext() {
    let iter = 0..3;
    let fbr = iter.into_fiber(11.1);

    let mut fbr = fbr.run().unwrap();
    assert_eq!(fbr.get(), Some(0));
    let mut fbr = fbr.run().unwrap();
    assert_eq!(fbr.get(), Some(1));
    let mut fbr = fbr.run().unwrap();
    assert_eq!(fbr.get(), Some(2));

    let st = fbr.run();
    assert_eq!(st.unwrap_done(), 11.1);
}

#[test]
fn fiber_iterator_ext_empty() {
    let iter = 0..0;
    let fbr = iter.into_fiber(11.1);

    let st = fbr.run();
    assert_eq!(st.unwrap_done(), 11.1);
}

#[test]
fn fiber_iterator_ext_lazy() {
    let iter = 0..3;
    let fbr = iter.into_fiber_lazy(|| 77.7);

    let mut fbr = fbr.run().unwrap();
    assert_eq!(fbr.get(), Some(0));
    let mut fbr = fbr.run().unwrap();
    assert_eq!(fbr.get(), Some(1));
    let mut fbr = fbr.run().unwrap();
    assert_eq!(fbr.get(), Some(2));

    let st = fbr.run();
    assert_eq!(st.unwrap_done(), 77.7);
}

#[test]
fn fiber_iterator_ext_lazy_empty() {
    let iter = 0..0;
    let fbr = iter.into_fiber_lazy(|| 11.1);

    let st = fbr.run();
    assert_eq!(st.unwrap_done(), 11.1);
}
