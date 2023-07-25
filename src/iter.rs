use super::{
    cofn::Eval,
    Fiber,
};

#[derive(Debug)]
pub struct IterFbr<I: Iterator> {
    iter: I,
    eval: Option<Eval<I::Item>>,
}

impl<I: Iterator> IterFbr<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            eval: None,
        }
    }
}

impl<I: Iterator> Fiber<I::Item> for IterFbr<I> {
    type Coro<'a> = &'a Eval<I::Item>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Coro<'_>> {
        self.eval = self.iter.next().map(Eval::from);
        self.eval.as_ref()
    }
}

pub trait FiberIterator: Iterator + Sized {
    fn into_fiber(self) -> IterFbr<Self> {
        IterFbr::new(self)
    }
}

impl<I> FiberIterator for I where I: Iterator + Sized {}
