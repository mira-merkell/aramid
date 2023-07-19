use std::mem;

use crate::{
    Fiber,
    State,
};

/// Iterator over yielded values of a fiber.
///
/// The fiber's final output is ignored.
pub struct FiberIter<F>
where
    F: Fiber,
{
    fbr: Option<F>,
}

impl<F: Fiber> FiberIter<F> {
    pub fn new(fbr: F) -> Self {
        Self {
            fbr: Some(fbr)
        }
    }
}

impl<F> Iterator for FiberIter<F>
where
    F: Fiber,
{
    type Item = F::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(fbr) = mem::take(&mut self.fbr) {
            match fbr.run() {
                State::Yield(mut yld) => {
                    let res = yld.get();
                    mem::swap(&mut self.fbr, &mut Some(yld));
                    Some(res)
                }
                State::Done(_) => None,
            }
        } else {
            None
        }
    }
}

pub(crate) struct FiberComplete<F, OP>
where
    F: Fiber,
    OP: FnMut(&mut F),
{
    fbr: Option<F>,
    f:   OP,
}

impl<F, OP> FiberComplete<F, OP>
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

impl<F, OP> Iterator for FiberComplete<F, OP>
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
