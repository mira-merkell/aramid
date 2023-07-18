//! Synthetic fibers.
//!
//! Fibers are little state machines that behave like coroutines: when run, they
//! yield first, and then they return.  In the meantime, they carry their full
//! stack around with them.
//!
//! Fibers are a model of concurrent computation.  They are static, lightweight
//! and particularly well-suited for cooperative multitasking.

use std::mem;

pub trait Fiber {
    type Output;
    type Yld: Yield<Fbr = Self>;

    fn run(self) -> State<Self>;

    fn into_iter(self) -> FiberIter<Self>
    where
        Self: Sized,
    {
        FiberIter::new(self)
    }
}

pub trait Yield {
    type Fbr: Fiber<Yld = Self>;

    fn fiber(self) -> Self::Fbr;

    fn get(&mut self) -> <Self::Fbr as Fiber>::Output;
}

pub enum State<F>
where
    F: Fiber + ?Sized,
{
    Yield(F::Yld),
    Done(F::Output),
}

impl<F> State<F>
where
    F: Fiber + ?Sized,
{
    /// # Panics
    ///
    /// Panics, if `State::Done`.
    pub fn unwrap_yield(self) -> F::Yld {
        match self {
            State::Yield(yld) => yld,
            State::Done(_) => panic!("state is Yield"),
        }
    }

    /// # Panics
    ///
    /// Panics, if `State::Yield`.
    pub fn unwrap_done(self) -> F::Output {
        match self {
            State::Yield(_) => panic!("state is Done"),
            State::Done(out) => out,
        }
    }

    pub fn is_yield(&self) -> bool {
        match self {
            State::Yield(_) => true,
            State::Done(_) => false,
        }
    }

    pub fn is_done(&self) -> bool {
        match self {
            State::Yield(_) => false,
            State::Done(_) => true,
        }
    }
}

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
    type Item = F::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(fbr) = mem::take(&mut self.fbr) {
            match fbr.run() {
                State::Yield(mut yld) => {
                    let res = yld.get();
                    mem::swap(&mut self.fbr, &mut Some(yld.fiber()));
                    Some(res)
                }
                State::Done(res) => Some(res),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Cubed(u64, u64);

    impl Cubed {
        fn new(n: u64) -> Self {
            Self(n, n)
        }
    }
    struct Squared(u64, Cubed);

    impl Yield for Squared {
        type Fbr = Cubed;

        fn fiber(self) -> Self::Fbr {
            self.1
        }

        fn get(&mut self) -> <Self::Fbr as Fiber>::Output {
            self.0
        }
    }

    impl Fiber for Cubed {
        type Output = u64;
        type Yld = Squared;

        fn run(mut self) -> State<Self> {
            if self.0 == self.1 {
                self.1 *= self.1;
                State::Yield(Squared(self.1, self))
            } else {
                State::Done(self.0 * self.1)
            }
        }
    }

    #[test]
    fn squared_01() {
        let fbr = Cubed::new(3);
        let state = fbr.run();
        let mut yld = state.unwrap_yield();
        assert_eq!(yld.get(), 9);

        let fbr = yld.fiber();
        let state = fbr.run();
        let out = state.unwrap_done();
        assert_eq!(out, 27);
    }

    #[test]
    fn squared_iter() {
        let fbr = Cubed::new(3);
        let res = fbr.into_iter().collect::<Vec<_>>();
        assert_eq!(res, &[9, 27]);
    }
}
