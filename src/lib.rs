//! Synthetic fibers 🧵
//!
//!
//! - _very much_ WIP 🚧
//! - Fibers are little state machines that behave like coroutines: when spun,
//!   they yield and yield, and then they return. In the meantime, they carry
//!   their stack around with them.
//! - Fibers are a model of concurrent computation. They are static, lightweight
//!   and well-suited for cooperative multitasking.
//!
//! The enum [`State`](crate::State) contains utility methods for processing
//! yielded values, not unlike the Standard Library's `Result` or `Option`.
//!
//! Additionally, fibers can be turned into iterators over their yielded
//! values; and closures that return a `State` can be turned into [fibers that
//! live on the heap](crate::HeapJob).

use std::mem;

pub trait Fiber
where
    Self: Sized,
{
    type Yield;
    type Output;

    /// Run the finder until it yields.
    fn run(self) -> State<Self>;

    /// Retrieve yielded value.
    fn get(&mut self) -> Self::Yield;

    /// Consume the fiber and turn it into an iterator over its yielded values.
    ///
    /// The final return value is ignored.
    fn into_iter(self) -> FiberIter<Self> {
        FiberIter::new(self)
    }

    /// Run the fiber to completion, discarding the yielded values.
    fn complete(self) -> Self::Output {
        FiberComplete::new(self).last().unwrap().unwrap()
    }
}

/// State of the fiber
#[derive(Debug, PartialEq)]
pub enum State<F>
where
    F: Fiber,
{
    /// Yielded value
    Yield(F),
    /// Done processing
    Done(F::Output),
}

impl<F> State<F>
where
    F: Fiber,
{
    /// # Panics
    ///
    /// Panics, if `State::Done`.
    pub fn unwrap(self) -> F {
        match self {
            State::Yield(fbr) => fbr,
            State::Done(_) => panic!("state is Yield"),
        }
    }

    /// # Panics
    ///
    /// Panics, if `State::Yield`.
    pub fn unwrap_done(self) -> <F as Fiber>::Output {
        match self {
            State::Yield(_) => panic!("state is Done"),
            State::Done(out) => out,
        }
    }

    /// Return true if state is `Yield`, otherwise return false.
    pub fn is_yield(&self) -> bool {
        match self {
            State::Yield(_) => true,
            State::Done(_) => false,
        }
    }

    /// Return true is state is `Done`, otherwise return false
    pub fn is_done(&self) -> bool {
        match self {
            State::Yield(_) => false,
            State::Done(_) => true,
        }
    }

    /// Return the value of `Done`, or apply operator `OP` on the value of
    /// `Yield`.
    pub fn done_or<OP>(
        self,
        f: OP,
    ) -> <F as Fiber>::Output
    where
        OP: FnOnce(F) -> F::Output,
    {
        match self {
            Self::Done(out) => out,
            Self::Yield(fbr) => f(fbr),
        }
    }

    /// Return the result of `OP` applied on the value of `Yield`
    ///
    /// wrapped in `Some`.  Return `None` is the value is `Done`.
    pub fn yield_and<OP, T>(
        self,
        f: OP,
    ) -> Option<T>
    where
        OP: FnOnce(F) -> T,
    {
        if let Self::Yield(fbr) = self {
            Some(f(fbr))
        } else {
            None
        }
    }

    /// Run fiber is the state is `Yield`.
    ///
    /// Returns the new yielded fiber wrapped in Some, or
    /// None, if the state was already `Done`.
    pub fn advance(self) -> Option<Self> {
        match self {
            Self::Done(_) => None,
            Self::Yield(fbr) => Some(fbr.run()),
        }
    }
}

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
            panic!("bug in iterator impl")
        }
    }
}

struct FiberComplete<F>
where
    F: Fiber,
{
    fbr: Option<F>,
}

impl<F: Fiber> FiberComplete<F> {
    fn new(fbr: F) -> Self {
        Self {
            fbr: Some(fbr)
        }
    }
}

impl<F> Iterator for FiberComplete<F>
where
    F: Fiber,
{
    type Item = Option<F::Output>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(fbr) = mem::take(&mut self.fbr) {
            match fbr.run() {
                State::Yield(yld) => {
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

/// A fiber consisting of a closure and continuation
///
/// The structure is allocated on the heap.
pub struct HeapJob<T> {
    f: Box<dyn FnOnce() -> State<HeapJob<T>>>,
}

impl<T> HeapJob<T> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce() -> State<HeapJob<T>> + 'static,
    {
        Self {
            f: Box::new(f)
        }
    }
}

impl<T> Fiber for HeapJob<T> {
    type Output = T;
    type Yield = ();

    fn run(self) -> State<Self> {
        (self.f)()
    }

    fn get(&mut self) -> Self::Yield {}
}

/// Specify the closure's continuation.
pub fn continue_with<T, OP>(f: OP) -> State<HeapJob<T>>
where
    OP: FnOnce() -> State<HeapJob<T>> + 'static,
{
    State::Yield(HeapJob::new(f))
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

    impl Fiber for Cubed {
        type Output = u64;
        type Yield = u64;

        fn run(mut self) -> State<Self> {
            if self.0 == self.1 {
                self.1 *= self.1;
                State::Yield(self)
            } else {
                State::Done(self.0 * self.1)
            }
        }

        fn get(&mut self) -> Self::Yield {
            self.1
        }
    }

    #[test]
    fn squared_01() {
        let fbr = Cubed::new(3);
        let state = fbr.run();
        let mut yld = state.unwrap();
        assert_eq!(yld.get(), 9);

        let fbr = yld;
        let state = fbr.run();
        let out = state.unwrap_done();
        assert_eq!(out, 27);
    }

    #[test]
    fn squared_iter() {
        let fbr = Cubed::new(3);
        let res = fbr.into_iter().collect::<Vec<_>>();
        assert_eq!(res, &[9,]);
    }

    #[test]
    fn squared_complete() {
        let fbr = Cubed::new(3);
        let res = fbr.complete();
        assert_eq!(res, 27);
    }

    #[test]
    fn heap_fiber_01() {
        let fbr = HeapJob::new(|| {
            println!("Hello from fiber");

            continue_with(|| {
                println!("Hello from continuation");
                State::Done(5)
            })
        });

        let fbr = fbr.run().unwrap();
        println!("Interlude");
        let res = fbr.run();
        assert_eq!(res.unwrap_done(), 5);
    }

    #[test]
    fn heap_fiber_02() {
        let fbr = HeapJob::new(|| {
            println!("Hello from fiber");

            continue_with(|| {
                println!("Hello from continuation 1");

                continue_with(|| {
                    println!("Hello from continuation 2");
                    State::Done(5)
                })
            })
        });

        let fbr = fbr.run().unwrap();
        println!("Interlude 1");

        let fbr = fbr.run().unwrap();
        println!("Interlude 2");

        let res = fbr.run();
        assert_eq!(res.unwrap_done(), 5);
    }
}
