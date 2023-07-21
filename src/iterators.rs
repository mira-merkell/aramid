//! `Iterator` ⇋ `Fiber` interface
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
//! Please note also that the method `Iterator::next()` takes the iterator by
//! mutable reference, whereas the analogous `Fiber::run()` consumes the fiber
//! and produces either a new one (or a modified version of itself), or the
//! final output wrapped in [`State`][state]
//!
//! [fiber-into-iter]: crate::Fiber::into_iter()
//! [fiber-iterator]: crate::FiberIterator
//! [std-iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
//! [state]: crate::State
use std::mem;

use crate::{
    Fiber,
    State,
};

/// Implementation of the [`Fiber`][fiber-trait] trait for
/// [`Iterators`][std-iterator].  
///
/// Typically, you wouldn't need to create this struct directly. Instead,
/// you can import the trait [`FiberIterator`][fiber-iterator-trait]
/// and call [`into_fiber()`][into-fiber] on any iterator.
///
/// [fiber-trait]: crate::Fiber
/// [std-iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
/// [fiber-iterator-trait]: crate::FiberIterator
/// [into-fiber]: crate::FiberIterator::into_fiber()
#[derive(Debug, PartialEq)]
pub struct FiberIter<I, T>
where
    I: Iterator,
{
    iter:   I,
    output: Option<T>,
}

impl<I, T> FiberIter<I, T>
where
    I: Iterator,
{
    pub fn new(
        iter: I,
        output: T,
    ) -> Self {
        Self {
            iter,
            output: Some(output),
        }
    }
}

impl<I, T> Fiber for FiberIter<I, T>
where
    I: Iterator,
{
    type Return = T;
    type Yield<'a> =  I::Item where Self: 'a;

    fn run(&mut self) -> State<I::Item, T> {
        match self.iter.next() {
            Some(val) => State::Yield(val),
            None => State::Done(mem::take(&mut self.output).unwrap()),
        }
    }
}

/// Implementation of the [`Fiber`][fiber-trait] trait for
/// [`Iterators`][std-iterator], evaluating lazily its final output.
///
/// Typically, you wouldn't need to create this struct directly. Instead,
/// you can import the trait [`FiberIterator`][fiber-iterator-trait]
/// and call [`into_fiber_lazy()`][into-fiber-lazy] on an any iterator.
///
/// [fiber-trait]: crate::Fiber
/// [std-iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
/// [fiber-iterator-trait]: crate::FiberIterator
/// [into-fiber-lazy]: crate::FiberIterator::into_fiber_lazy()
#[derive(Debug, PartialEq)]
pub struct FiberIterLazy<I, T, OP>
where
    I: Iterator,
    OP: Fn() -> T,
{
    iter: I,
    f:    OP,
}

impl<I, T, OP> FiberIterLazy<I, T, OP>
where
    I: Iterator,
    OP: Fn() -> T,
{
    pub fn new(
        iter: I,
        f: OP,
    ) -> Self {
        Self {
            iter,
            f,
        }
    }
}

impl<I, T, OP> Fiber for FiberIterLazy<I, T, OP>
where
    I: Iterator,
    OP: Fn() -> T,
{
    type Return = T;
    type Yield<'a> = I::Item where Self: 'a;

    fn run(&mut self) -> State<I::Item, T> {
        match self.iter.next() {
            Some(val) => State::Yield(val),
            None => State::Done((self.f)()),
        }
    }
}

/// Extension trait fo turning iterators into fibers.
pub trait FiberIterator: Iterator + Sized {
    /// Consume iterator and create a fiber that will yield values
    /// produced by the iterator.
    ///
    /// The fiber's final output is given as argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use aramid::Fiber;
    /// use aramid::FiberIterator;
    ///
    /// let output = 55.5;
    /// let fiber = (0..3).into_fiber(output);
    ///
    /// let mut coll = Vec::new();
    /// let result = fiber.complete(|fbr| coll.push(fbr.get()));
    ///
    /// assert_eq!(coll, &[Some(0), Some(1), Some(2)]);
    /// assert_eq!(result, 55.5);
    /// ```
    fn into_fiber<T>(
        self,
        output: T,
    ) -> FiberIter<Self, T> {
        FiberIter::new(self, output)
    }

    /// Consume iterator and create a fiber that will yield values
    /// produced by the iterator.
    ///
    /// The fiber's final output is lazily evaluated at the end of iteration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use aramid::Fiber;
    /// use aramid::FiberIterator;
    ///
    /// let output = 55.5;
    /// let fiber = (0..3).into_fiber_lazy(|| output == 3.14);
    ///
    /// let mut coll = Vec::new();
    /// let result = fiber.complete(|fbr| coll.push(fbr.get()));
    ///
    /// assert_eq!(coll, &[Some(0), Some(1), Some(2)]);
    /// assert_eq!(result, false);
    /// ```
    fn into_fiber_lazy<T, OP>(
        self,
        f: OP,
    ) -> FiberIterLazy<Self, T, OP>
    where
        OP: Fn() -> T,
    {
        FiberIterLazy::new(self, f)
    }
}

impl<T> FiberIterator for T where T: Iterator + Sized {}
