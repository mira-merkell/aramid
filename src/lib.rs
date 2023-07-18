//! Synthetic fibers 🧵
//!
//!
//! - _very much_ WIP 🚧
//! - Fibers are little state machines that behave like coroutines: when spun,
//!   they yield and yield, and then they return. In the meantime, they carry
//!   their full stack around with them.
//! - Fibers are a model of concurrent computation. They are static, lightweight
//!   and particularly well-suited for cooperative multitasking.
//!
//! The API is built around two tied up traits: [`Fiber`](crate::Fiber) and
//! [`Yield`](crate::Yield): a type implementing `Fiber` must have an associated
//! type that implements `Yield` whose associated type, in turn, must be the
//! original type itself.  This way, a type that implements `Fiber` becomes
//! automatically a state machine: calling `Fiber::run()` produces
//! `Yield` wrapped in [`State::Yield`](crate::State) that can elongate the
//! fiber by calling `Yield::fiber()`.  When the fiber is finished, the last
//! call to run will produce `State::Done` variant from which the final result
//! can be extracted.
//!
//! Each instance of `Yield` can yield an additional value that need not to be
//! the same as the type of the final output of the fiber.
//!
//! The enum [`State`](crate::State) contains utility methods for processing
//! yielded values, not unlike the Standard Library's `Result` or `Option`.
//!
//! Additionally, sized fibers can be turned into iterators over their yielded
//! values; and closures that return a special type:
//! [`Continuation`](crate::Continuation) can be turned into [fibers that live
//! on the heap](crate::HeapFiber), much like standard coroutines in other
//! languages.

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

    /// Run the fiber to completion, discarding the yielded values.
    fn complete(self)
    where
        Self: Sized,
    {
        let _ = self.into_iter().last();
    }
}

pub trait Yield {
    type Output;
    type Fbr: Fiber<Yld = Self>;

    fn fiber(self) -> Self::Fbr;

    fn get(&mut self) -> Self::Output;
}

#[derive(Debug, PartialEq)]
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
    /// Panics, if `State::Yield`.
    pub fn unwrap_done(self) -> <F as Fiber>::Output {
        match self {
            State::Yield(_) => panic!("state is Done"),
            State::Done(out) => out,
        }
    }

    /// # Panics
    ///
    /// Panics, if `State::Done`.
    pub fn unwrap_yield(self) -> F::Yld {
        match self {
            State::Yield(yld) => yld,
            State::Done(_) => panic!("state is Yield"),
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

    pub fn done_or<OP>(
        self,
        f: OP,
    ) -> <F as Fiber>::Output
    where
        OP: FnOnce(F::Yld) -> F::Output,
    {
        match self {
            Self::Done(out) => out,
            Self::Yield(yld) => f(yld),
        }
    }

    pub fn yield_and<OP, T>(
        self,
        f: OP,
    ) -> Option<T>
    where
        OP: FnOnce(F::Yld) -> T,
    {
        if let Self::Yield(yld) = self {
            Some(f(yld))
        } else {
            None
        }
    }

    pub fn advance(self) -> Option<Self>
    where
        F: Sized,
    {
        match self {
            Self::Done(_) => None,
            Self::Yield(yld) => Some(yld.fiber().run()),
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

/// Iterator over yielded values of a fiber.
///
/// The fiber's final output is ignored.
impl<F> Iterator for FiberIter<F>
where
    F: Fiber,
{
    type Item = <F::Yld as Yield>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(fbr) = mem::take(&mut self.fbr) {
            match fbr.run() {
                State::Yield(mut yld) => {
                    let res = yld.get();
                    mem::swap(&mut self.fbr, &mut Some(yld.fiber()));
                    Some(res)
                }
                State::Done(_) => None,
            }
        } else {
            panic!("bug in iterator impl")
        }
    }
}

pub enum Continuation<T, K> {
    Yield(HeapYield<T, K>),
    Done(T),
}

pub struct HeapYield<T, K> {
    val: Option<K>,
    fbr: HeapFiber<T, K>,
}

impl<T, K> HeapYield<T, K> {
    pub fn new(
        val: K,
        fbr: HeapFiber<T, K>,
    ) -> Self {
        Self {
            fbr,
            val: Some(val),
        }
    }
}

pub struct HeapFiber<T, K> {
    f: Box<dyn FnOnce() -> Continuation<T, K>>,
}

impl<T, K> HeapFiber<T, K> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce() -> Continuation<T, K> + 'static,
    {
        Self {
            f: Box::new(f)
        }
    }
}

impl<T, K> Fiber for HeapFiber<T, K> {
    type Output = T;
    type Yld = HeapYield<T, K>;

    fn run(self) -> State<Self> {
        match (self.f)() {
            Continuation::Done(res) => State::Done(res),
            Continuation::Yield(yld) => State::Yield(yld),
        }
    }
}

impl<T, K> Yield for HeapYield<T, K> {
    type Fbr = HeapFiber<T, K>;
    type Output = K;

    fn fiber(self) -> Self::Fbr {
        self.fbr
    }

    fn get(&mut self) -> Self::Output {
        mem::take(&mut self.val).unwrap()
    }
}

pub fn continue_with<T, K, OP>(
    val: K,
    f: OP,
) -> Continuation<T, K>
where
    OP: FnOnce() -> Continuation<T, K> + 'static,
{
    Continuation::Yield(HeapYield::new(val, HeapFiber::new(f)))
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
        type Output = u64;

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
        assert_eq!(res, &[9,]);
    }

    #[test]
    fn heap_fiber_01() {
        let fbr = HeapFiber::new(|| {
            println!("Hello from fiber");

            continue_with(55.5, || {
                println!("Hello from continuation");
                Continuation::Done(5)
            })
        });

        let mut yld = fbr.run().unwrap_yield();
        assert_eq!(yld.get(), 55.5);
        println!("Interlude");
        let res = yld.fiber().run();
        assert_eq!(res.unwrap_done(), 5);
    }

    #[test]
    fn heap_fiber_02() {
        let fbr = HeapFiber::new(|| {
            println!("Hello from fiber");

            continue_with(55.5, || {
                println!("Hello from continuation 1");

                continue_with(44.4, || {
                    println!("Hello from continuation 2");
                    Continuation::Done(5)
                })
            })
        });

        let mut yld = fbr.run().unwrap_yield();
        assert_eq!(yld.get(), 55.5);
        println!("Interlude 1");

        let mut yld = yld.fiber().run().unwrap_yield();
        assert_eq!(yld.get(), 44.4);
        println!("Interlude 2");

        let res = yld.fiber().run();
        assert_eq!(res.unwrap_done(), 5);
    }
}
