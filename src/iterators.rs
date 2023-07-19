//! `Iterator` ⇋ `Fiber` interface

use std::mem;

use crate::{
    Fiber,
    State,
};

/// Iterator over values yielded by a fiber.
///
/// The fiber's final output is given to as argument
/// to the supplied closure.
pub struct Iter<F, OP>
where
    F: Fiber,
    OP: FnMut(F::Output),
{
    fbr: Option<F>,
    f:   OP,
}

impl<F, OP> Iter<F, OP>
where
    F: Fiber,
    OP: FnMut(F::Output),
{
    pub fn new(
        fbr: F,
        f: OP,
    ) -> Self {
        Self {
            fbr: Some(fbr),
            f,
        }
    }
}

impl<F, OP> Iterator for Iter<F, OP>
where
    F: Fiber,
    OP: FnMut(F::Output),
{
    type Item = F::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(fbr) = mem::take(&mut self.fbr) {
            match fbr.run() {
                State::Yield(mut yld) => {
                    let res = yld.get();
                    mem::swap(&mut self.fbr, &mut Some(yld));
                    res
                }
                State::Done(res) => {
                    (self.f)(res);
                    None
                }
            }
        } else {
            None
        }
    }
}

pub(crate) struct IterComplete<F, OP>
where
    F: Fiber,
    OP: FnMut(&mut F),
{
    fbr: Option<F>,
    f:   OP,
}

impl<F, OP> IterComplete<F, OP>
where
    F: Fiber,
    OP: FnMut(&mut F),
{
    pub(crate) fn new(
        fbr: F,
        f: OP,
    ) -> Self {
        Self {
            fbr: Some(fbr),
            f,
        }
    }
}

impl<F, OP> Iterator for IterComplete<F, OP>
where
    F: Fiber,
    OP: FnMut(&mut F),
{
    type Item = Option<F::Output>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(fbr) = mem::take(&mut self.fbr) {
            match fbr.run() {
                State::Yield(mut yld) => {
                    (self.f)(&mut yld);
                    mem::swap(&mut self.fbr, &mut Some(yld));
                    Some(None)
                }
                State::Done(res) => Some(Some(res)),
            }
        } else {
            None
        }
    }
}

pub struct FiberIter<I, K>
where
    I: Iterator,
{
    iter:   I,
    val:    Option<I::Item>,
    output: K,
}

impl<I, K> FiberIter<I, K>
where
    I: Iterator,
{
    pub fn new(
        iter: I,
        output: K,
    ) -> Self {
        Self {
            iter,
            val: None,
            output,
        }
    }
}

impl<I, K> Fiber for FiberIter<I, K>
where
    I: Iterator,
{
    type Output = K;
    type Yield = I::Item;

    fn run(mut self) -> State<Self> {
        self.val = self.iter.next();
        match self.val {
            Some(_) => State::Yield(self),
            None => State::Done(self.output),
        }
    }

    fn get(&mut self) -> Option<<I as Iterator>::Item> {
        mem::take(&mut self.val)
    }
}

pub struct FiberIterLazy<I, K, OP>
where
    I: Iterator,
    OP: FnOnce() -> K,
{
    iter: I,
    val:  Option<I::Item>,
    f:    OP,
}

impl<I, K, OP> FiberIterLazy<I, K, OP>
where
    I: Iterator,
    OP: FnOnce() -> K,
{
    pub fn new(
        iter: I,
        f: OP,
    ) -> Self {
        Self {
            iter,
            val: None,
            f,
        }
    }
}

impl<I, K, OP> Fiber for FiberIterLazy<I, K, OP>
where
    I: Iterator,
    OP: FnOnce() -> K,
{
    type Output = K;
    type Yield = I::Item;

    fn run(mut self) -> State<Self> {
        self.val = self.iter.next();
        match self.val {
            Some(_) => State::Yield(self),
            None => State::Done((self.f)()),
        }
    }

    fn get(&mut self) -> Option<<I as Iterator>::Item> {
        mem::take(&mut self.val)
    }
}

/// Extension trait turning iterators into fibers.
pub trait FiberIterator: Iterator + Sized {
    /// Consume iterator and create a fiber that will yield values
    /// produced by the iterator.
    ///
    /// The fiber's final output is given as argument.
    fn into_fiber<K>(
        self,
        output: K,
    ) -> FiberIter<Self, K> {
        FiberIter::new(self, output)
    }

    /// Consume iterator and create a fiber that will yield values
    /// produced by the iterator.
    ///
    /// The fiber's final output is lazily evaluated at the end of iteration.
    fn into_fiber_lazy<K, OP>(
        self,
        f: OP,
    ) -> FiberIterLazy<Self, K, OP>
    where
        OP: FnOnce() -> K,
    {
        FiberIterLazy::new(self, f)
    }
}

impl<T> FiberIterator for T where T: Iterator + Sized {}

#[test]
fn iterator_ext_trait() {
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
fn iterator_ext_trait_lazy() {
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
