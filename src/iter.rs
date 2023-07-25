use super::{
    cofn::Eval,
    Fiber,
};

#[derive(Debug)]
pub struct IntoFiber<I: Iterator> {
    iter: I,
}

impl<I: Iterator> IntoFiber<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
        }
    }
}

impl<I: Iterator> Fiber<I::Item> for IntoFiber<I> {
    type Coro<'a> =  Eval<I::Item>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Coro<'_>> {
        self.iter.next().map(Eval::from)
    }
}

pub trait FiberIterator: Iterator + Sized {
    fn into_fiber(self) -> IntoFiber<Self> {
        IntoFiber::new(self)
    }
}

impl<I> FiberIterator for I where I: Iterator + Sized {}
