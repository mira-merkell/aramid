//! Synthetic fibers 🧵
//!
//! - _very much_ WIP 🚧.
//! - [Fibers](https://en.wikipedia.org/wiki/Fiber_(computer_science)) are
//!   little state machines that behave like coroutines: when spun, they yield
//!   and yield, and then they return. In the meantime, they carry their stack
//!   around with them.
//! - Fibers are a model of concurrent computation. They are static, lightweight
//!   and well-suited for cooperative multitasking.
//!
//! The enum [`State`](crate::State) contains utility methods for processing
//! yielded values, not unlike the Standard Library's `Result` or `Option`.
//! Closures that return a `State` can be turned into [fibers that live on the
//! heap](crate::HeapJob).
//!
//! ## Fibers and Iterators
//!
//! Additionally, fibers can be turned into iterators over their yielded
//! values...
//!
//! See [`iterators`][module-iterators] for more details.
//!
//! [module-iterators]: crate::iterators

pub mod iterators;
pub use iterators::FiberIterator;
use iterators::{
    Iter,
    IterComplete,
};

/// Lightweight coroutines for cooperative multitasking.
pub trait Fiber
where
    Self: Sized,
{
    type Yield;
    type Output;

    /// Run the fiber until it yields.
    fn run(self) -> State<Self>;

    /// Retrieve the yielded value.
    fn get(&mut self) -> Self::Yield;

    /// Consume the fiber and turn it into an iterator over its yielded values.
    ///
    /// The fiber's final output is given to the supplied closure
    /// as an argument.
    fn into_iter<OP>(
        self,
        f: OP,
    ) -> Iter<Self, OP>
    where
        OP: FnMut(Self::Output),
    {
        Iter::new(self, f)
    }

    /// Run the fiber to completion.
    ///
    /// Call `OP` on each of the yielded fibers.  Return final output.
    fn complete<OP>(
        self,
        f: OP,
    ) -> Self::Output
    where
        OP: FnMut(&mut Self),
    {
        IterComplete::new(self, f).last().unwrap().unwrap()
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

    /// Return true is state is `Done`, otherwise return false.
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

    /// Return the result of `OP` applied on the value of `Yield`.
    ///
    /// Return `None` is the value is `Done`.
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

    /// Run fiber if the state is `Yield`.
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

/// A fiber consisting of a closure and continuation.
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
