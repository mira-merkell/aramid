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
    ///
    /// Returns  `None` if the value was `Yield`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use aramid::{Fiber, State, FiberIterator};
    /// let output = 11.5;
    ///
    /// // This fiber will yield: 0, and then output: 11.5.
    /// let fiber = (0..1).into_fiber(output);
    /// let mut state = fiber.run();
    ///
    /// let mut peek = None;
    /// let check = state.done_or(|fbr| peek = Some(fbr.get()));
    /// assert_eq!(check, None);
    /// assert_eq!(peek, Some(0));
    ///
    /// let fiber = state.unwrap();
    /// let mut state = fiber.run();
    ///
    /// assert_eq!(state, State::Done(output));
    /// assert_eq!(*state.done_or(|_| ()).unwrap(), output);
    /// ```
    pub fn done_or<OP>(
        &mut self,
        f: OP,
    ) -> Option<&mut F::Output>
    where
        OP: FnOnce(&mut F),
    {
        match self {
            Self::Done(out) => Some(out),
            Self::Yield(fbr) => {
                f(fbr);
                None
            }
        }
    }

    /// Return the result of `OP` applied on the value of `Yield`.
    ///
    /// Return `None` if the value is `Done`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aramid::{Fiber, State, FiberIterator};
    /// let output = 11.5;
    ///
    /// // This fiber will yield: 0, and then output: 11.5.
    /// let fiber = (0..1).into_fiber(output);
    /// let mut state = fiber.run();
    ///
    /// assert!(state.yield_and(|fbr| fbr.get() == 0).unwrap());
    ///
    /// let fiber = state.unwrap();
    /// let mut state = fiber.run();
    ///
    /// assert_eq!(state, State::Done(output));
    /// assert_eq!(state.yield_and(|fbr| fbr.get() == 0), None);
    /// ```
    pub fn yield_and<OP, T>(
        &mut self,
        f: OP,
    ) -> Option<T>
    where
        OP: FnOnce(&mut F) -> T,
    {
        if let Self::Yield(fbr) = self {
            Some(f(fbr))
        } else {
            None
        }
    }

    /// Run the fiber to completion.
    ///
    /// If the value is `Done`, return it immediately, otherwise
    /// call `OP` on each of the yielded fibers.  Return final output.
    pub fn complete<OP>(
        self,
        f: OP,
    ) -> F::Output
    where
        OP: FnMut(&mut F),
    {
        match self {
            Self::Done(res) => res,
            Self::Yield(fbr) => fbr.complete(f),
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
