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
//! Please note also that the method `Iterator::next()` take the iterator by
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

/// Iterator over values yielded by a fiber.
///
/// The fiber's final output is given to the supplied closure as an argument.
///
/// This iterator is usually created by calling
/// [`Fiber::into_inter()`][fiber-into_iter].  See that method for more details.
///
/// [fiber-into_iter]: crate::Fiber::into_iter()
pub struct Iter<F, OP>
where
    F: Fiber,
    OP: FnMut(F::Output),
{
    fbr: Option<F>,
    f:   OP,
}

impl<F, OP> Iter<F, OP>
where
    F: Fiber,
    OP: FnMut(F::Output),
{
    pub fn new(
        fbr: F,
        f: OP,
    ) -> Self {
        Self {
            fbr: Some(fbr),
            f,
        }
    }
}

impl<F, OP> Iterator for Iter<F, OP>
where
    F: Fiber,
    OP: FnMut(F::Output),
{
    type Item = F::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(fbr) = mem::take(&mut self.fbr) {
            match fbr.run() {
                State::Yield(mut yld) => {
                    let res = yld.get();
                    mem::swap(&mut self.fbr, &mut Some(yld));
                    res
                }
                State::Done(res) => {
                    (self.f)(res);
                    None
                }
            }
        } else {
            None
        }
    }
}

// Iterator used to implement `Fiber::complete()`.
pub(crate) struct IterComplete<F, OP>
where
    F: Fiber,
    OP: FnMut(&mut F),
{
    fbr: Option<F>,
    f:   OP,
}

impl<F, OP> IterComplete<F, OP>
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

impl<F, OP> Iterator for IterComplete<F, OP>
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

/// Implementation of the [`Fiber`][fiber-trait] trait for
/// [`Iterators`][std-iterator].  
///
/// Typically, you wouldn't need to create this struct directly. Instead,
/// you can import the trait [`FiberIterator`][fiber-iterator-trait]
/// and call [`into_fiber()`][into-fiber] on an iterator directly.
///
/// [fiber-trait]: crate::Fiber
/// [std-iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
/// [fiber-iterator-trait]: crate::FiberIterator
/// [into-fiber]: crate::FiberIterator::into_fiber()
pub struct FiberIter<I, K>
where
    I: Iterator,
{
    iter:   I,
    val:    Option<I::Item>,
    output: K,
}

impl<I, K> FiberIter<I, K>
where
    I: Iterator,
{
    pub fn new(
        iter: I,
        output: K,
    ) -> Self {
        Self {
            iter,
            val: None,
            output,
        }
    }
}

impl<I, K> Fiber for FiberIter<I, K>
where
    I: Iterator,
{
    type Output = K;
    type Yield = I::Item;

    fn run(mut self) -> State<Self> {
        self.val = self.iter.next();
        match self.val {
            Some(_) => State::Yield(self),
            None => State::Done(self.output),
        }
    }

    fn get(&mut self) -> Option<<I as Iterator>::Item> {
        mem::take(&mut self.val)
    }
}

/// Implementation of the [`Fiber`][fiber-trait] trait for
/// [`Iterators`][std-iterator], evaluating lazily its final output.
///
/// Typically, you wouldn't need to create this struct directly. Instead,
/// you can import the trait [`FiberIterator`][fiber-iterator-trait]
/// and call [`into_fiber_lazy()`][into-fiber-lazy] on an iterator directly.
///
/// [fiber-trait]: crate::Fiber
/// [std-iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
/// [fiber-iterator-trait]: crate::FiberIterator
/// [into-fiber-lazy]: crate::FiberIterator::into_fiber_lazy()
pub struct FiberIterLazy<I, K, OP>
where
    I: Iterator,
    OP: FnOnce() -> K,
{
    iter: I,
    val:  Option<I::Item>,
    f:    OP,
}

impl<I, K, OP> FiberIterLazy<I, K, OP>
where
    I: Iterator,
    OP: FnOnce() -> K,
{
    pub fn new(
        iter: I,
        f: OP,
    ) -> Self {
        Self {
            iter,
            val: None,
            f,
        }
    }
}

impl<I, K, OP> Fiber for FiberIterLazy<I, K, OP>
where
    I: Iterator,
    OP: FnOnce() -> K,
{
    type Output = K;
    type Yield = I::Item;

    fn run(mut self) -> State<Self> {
        self.val = self.iter.next();
        match self.val {
            Some(_) => State::Yield(self),
            None => State::Done((self.f)()),
        }
    }

    fn get(&mut self) -> Option<<I as Iterator>::Item> {
        mem::take(&mut self.val)
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
    fn into_fiber<K>(
        self,
        output: K,
    ) -> FiberIter<Self, K> {
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
    fn into_fiber_lazy<K, OP>(
        self,
        f: OP,
    ) -> FiberIterLazy<Self, K, OP>
    where
        OP: FnOnce() -> K,
    {
        FiberIterLazy::new(self, f)
    }
}

impl<T> FiberIterator for T where T: Iterator + Sized {}
