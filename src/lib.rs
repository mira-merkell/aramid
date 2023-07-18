//! Synthetic fibers.
//!
//! Fibers are little state machines that behave like coroutines: when spun out,
//! they yield first, and then they return.  In the meantime, they carry their
//! full stack around with them.
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
        Self::Yld: Yield<Output = Self::Output>,
    {
        FiberIter::new(self)
    }

    /// Run the fiber to completion, discarding the yielded values.
    fn complete(self) -> Self::Output
    where
        Self: Sized,
        Self::Yld: Yield<Output = Self::Output>,
    {
        self.into_iter().last().unwrap()
    }
}

pub trait Yield {
    type Fbr: Fiber<Yld = Self>;
    type Output;

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

impl<F> Iterator for FiberIter<F>
where
    F: Fiber,
    F::Yld: Yield<Output = F::Output>,
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
        assert_eq!(res, &[9, 27]);
    }

    #[test]
    fn heap_fiber_01() {
        let fbr = HeapFiber::new(|| {
            println!("Hello from fiber");
            Continuation::Yield(HeapYield::new(
                55.5,
                HeapFiber::new(|| {
                    println!("Hello from continuation");
                    Continuation::Done(5)
                }),
            ))
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
            Continuation::Yield(HeapYield::new(
                55.5,
                HeapFiber::new(|| {
                    println!("Hello from continuation 1");
                    Continuation::Yield(HeapYield::new(
                        44.4,
                        HeapFiber::new(|| {
                            println!("Hello from continuation 2");
                            Continuation::Done(5)
                        }),
                    ))
                }),
            ))
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
