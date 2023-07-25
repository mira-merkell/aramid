use super::{
    cofn::Yield,
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
    type Coro<'a> =  Yield<I::Item>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Coro<'_>> {
        self.iter.next().map(Yield::from)
    }
}

pub trait FiberIterator: IntoIterator + Sized {
    fn into_fiber(self) -> IntoFiber<Self::IntoIter> {
        IntoFiber::new(self.into_iter())
    }
}

impl<I> FiberIterator for I where I: IntoIterator + Sized {}
