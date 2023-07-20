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
//! The library provides a convenient interface between fibers and iterators.
//! On the one hand, there is [`Fiber::into_iter()`][fiber-into-iter] method
//! that consumes the fiber and return an iterator over its yielded values;
//! on the other, any iterator can be easily turned into a fiber by invoking
//! `into_fiber()` or `into_fiber_lazy()` from the extension trait
//! [`FiberIterator`][fiber-iterator].
//!
//! The main difference between fibers and iterators is that the `Fiber` trait
//! specifies *two* associated types: `Yield` and `Output`, whereas in order to
//! implement [`Iterator`][std-iterator] only one type: `Item` suffices.  Thanks
//! to that, fibers producing different types can be easily chained into
//! powerful state machines.
//!
//! See [`iterators`][module-iterators] module for more details.
//!
//! [fiber-into-iter]: crate::Fiber::into_iter()
//! [fiber-iterator]: crate::FiberIterator
//! [std-iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
//! [module-iterators]: crate::iterators

pub mod iterators;

pub use iterators::FiberIterator;
// use iterators::LendingIter;

/// Lightweight coroutines for cooperative multitasking.
pub trait Fiber {
    /// The type of the yielded values.
    type Yield<'a>
    where
        Self: 'a;
    /// The type of the final output produced by the fiber.
    type Output;

    /// Run the fiber until it yields.
    fn run(&mut self) -> State<Self::Yield<'_>, Self::Output>;

    /// Run the fiber to completion.
    ///
    /// Call `OP` on each of the yielded values.  Return the final output.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aramid::{Fiber, FiberIterator};
    /// let output = 55.5;
    /// let fiber = (0..3).into_fiber(output);
    ///
    /// let mut coll = Vec::new();
    /// let result = fiber.complete(|x| coll.push(x.get()));
    ///
    /// assert_eq!(coll, &[Some(0), Some(1), Some(2)]);
    /// assert_eq!(result, 55.5);
    /// ```
    fn complete<OP>(
        &mut self,
        mut f: OP,
    ) -> Self::Output
    where
        OP: FnMut(Self::Yield<'_>),
    {
        loop {
            match self.run() {
                State::Yield(yld) => {
                    (f)(yld);
                }
                State::Done(res) => break res,
            }
        }
    }
}

/// State of the fiber
#[derive(Debug, PartialEq)]
#[must_use]
pub enum State<Y, T> {
    /// Yielded value
    Yield(Y),
    /// Done processing
    Done(T),
}

impl<Y, T> State<Y, T> {
    /// Unwrap the value wrapped in `Yield`.
    ///
    /// # Panics
    ///
    /// This function panics, if the state is `Done`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use aramid::{Fiber, State, FiberIterator};
    /// let output = ();
    /// let fiber = (0..1).into_fiber(output);
    ///
    /// let fiber = fiber.run().unwrap();
    /// ```
    pub fn unwrap(self) -> Y {
        match self {
            State::Yield(yld) => yld,
            State::Done(_) => panic!("state is Yield"),
        }
    }

    /// Unwrap the value wrapped in `Done`.
    ///
    /// # Panics
    ///
    /// This function panics, if the state is `Yield`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use aramid::{Fiber, State, FiberIterator};
    /// let output = 55.5;
    /// let fiber = (0..1).into_fiber(output);
    ///
    /// let fiber = fiber.run().unwrap();
    /// let result = fiber.run().unwrap_done();
    ///
    /// assert_eq!(result, 55.5);
    /// ```
    pub fn unwrap_done(self) -> T {
        match self {
            State::Yield(_) => panic!("state is Done"),
            State::Done(out) => out,
        }
    }

    /// Return true, if the state is `Yield`, otherwise return false.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use aramid::{Fiber, State, FiberIterator};
    /// let output = ();
    /// let fiber = (0..1).into_fiber(output);
    ///
    /// let state = fiber.run();
    /// assert!(state.is_yield());
    ///
    /// let new_state = state.advance();
    /// assert!(new_state.is_done());
    /// ```
    pub fn is_yield(&self) -> bool {
        match self {
            State::Yield(_) => true,
            State::Done(_) => false,
        }
    }

    /// Return true, if the state is `Done`, otherwise return false.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use aramid::{Fiber, State, FiberIterator};
    /// let output = ();
    /// let fiber = (0..1).into_fiber(output);
    ///
    /// let state = fiber.run();
    /// assert!(state.is_yield());
    ///
    /// let new_state = state.advance();
    /// assert!(new_state.is_done());
    /// ```
    pub fn is_done(&self) -> bool {
        match self {
            State::Yield(_) => false,
            State::Done(_) => true,
        }
    }

    // /// Return the value of `Done`, or apply operator `OP` on the value of
    // /// `Yield`.
    // ///
    // /// Returns  `None` if the value was `Yield`.
    // ///
    // /// # Examples
    // ///
    // /// ```rust
    // /// # use aramid::{Fiber, State, FiberIterator};
    // /// let output = 11.5;
    // ///
    // /// // This fiber will yield: 0, and then output: 11.5.
    // /// let fiber = (0..1).into_fiber(output);
    // /// let mut state = fiber.run();
    // ///
    // /// let mut peek = None;
    // /// let check = state.done_or(|fbr| peek = fbr.get());
    // /// assert_eq!(check, None);
    // /// assert_eq!(peek, Some(0));
    // ///
    // /// let fiber = state.unwrap();
    // /// let mut state = fiber.run();
    // ///
    // /// assert_eq!(state, State::Done(output));
    // /// assert_eq!(*state.done_or(|_| ()).unwrap(), output);
    // /// ```
    // pub fn done_or<OP>(
    //     &mut self,
    //     f: OP,
    // ) -> Option<&mut F::Output>
    // where
    //     OP: FnOnce(&mut F),
    // {
    //     match self {
    //         Self::Output(out) => Some(out),
    //         Self::Yield(fbr) => {
    //             f(fbr);
    //             None
    //         }
    //     }
    // }

    // /// Return the result of `OP` applied on the value of `Yield`.
    // ///
    // /// Return `None` if the value is `Done`.
    // ///
    // /// # Example
    // ///
    // /// ```rust
    // /// # use aramid::{Fiber, State, FiberIterator};
    // /// let output = 11.5;
    // ///
    // /// // This fiber will yield: 0, and then output: 11.5.
    // /// let fiber = (0..1).into_fiber(output);
    // /// let mut state = fiber.run();
    // ///
    // /// assert!(state.yield_and(|fbr| fbr.get() == Some(0)).unwrap());
    // ///
    // /// let fiber = state.unwrap();
    // /// let mut state = fiber.run();
    // ///
    // /// assert_eq!(state, State::Done(output));
    // /// assert_eq!(state.yield_and(|fbr| fbr.get() == Some(0)), None);
    // /// ```
    // pub fn yield_and<OP, T>(
    //     &mut self,
    //     f: OP,
    // ) -> Option<T>
    // where
    //     OP: FnOnce(&mut F) -> T,
    // {
    //     if let Self::Yield(fbr) = self {
    //         Some(f(fbr))
    //     } else {
    //         None
    //     }
    // }
}

// /// A fiber consisting of a closure and continuation.
// ///
// /// The structure is allocated on the heap.
// pub struct HeapJob<T> {
//     f: Option<Box<dyn FnOnce() -> State<HeapJob<T>>>>,
// }

// impl<T> HeapJob<T> {
//     pub fn new<F>(f: F) -> Self
//     where
//         F: FnOnce() -> State<HeapJob<T>> + 'static,
//     {
//         Self {
//             f: Some(Box::new(f))
//         }
//     }

// }

// impl<T> Fiber for HeapJob<T> {
//     type Output = T;
//     type Yield = ();

//     fn run(&mut self) -> State<Self> {
//         let f = mem::take(&mut self.f).unwrap();
//         match f() {
//             State::Output(res) => State::Output(res),
//             State::Yield(f) => {
//                 mem::replace(&mut self.f, f);
//                 S
//             }
//         }

//     }

//     // fn get(&mut self) -> Option<Self::Yield> {
//     //     None
//     // }
// }

// /// Specify the closure's continuation.
// pub fn continue_with<T, OP>(f: OP) -> State<HeapJob<T>>
// where
//     OP: FnOnce() -> State<HeapJob<T>> + 'static,
// {
//     State::Yield(HeapJob::new(f))
// }
