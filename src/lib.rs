//! Synthetic fibers.
//!
//! Fibers are little state machines that behave like coroutines: when run, they
//! yield first, and then they return.  In the meantime, they carry their full
//! stack around with them.
//!
//! Fibers are a model of concurrent computation.  They are static, lightweight
//! and particularly well-suited for cooperative multitasking.

pub enum FiberState<F>
where
    F: Fiber,
{
    Pending(F::Yield, F),
    Done(F::Final),
}

pub trait Fiber
where
    Self: Sized,
{
    type Yield;
    type Final;

    fn run(self) -> FiberState<Self>;
}

/// Continuation
pub enum Ctn<Y, R> {
    Pending(Y, BoxedFiber<Y, R>),
    Done(R),
}

pub struct BoxedFiber<Y, F> {
    f: Box<dyn FnOnce() -> Ctn<Y, F>>,
}

impl<Y, F> BoxedFiber<Y, F> {
    pub fn new<OP>(f: OP) -> Self
    where
        OP: FnOnce() -> Ctn<Y, F> + 'static,
    {
        let f = Box::new(f);
        Self {
            f,
        }
    }
}

impl<Y, F> Fiber for BoxedFiber<Y, F> {
    type Final = F;
    type Yield = Y;

    fn run(self) -> FiberState<Self> {
        match (self.f)() {
            Ctn::Pending(y, fibr) => FiberState::Pending(y, fibr),
            Ctn::Done(res) => FiberState::Done(res),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boxed_fiber_01() {
        let fibr = BoxedFiber::new(|| {
            println!("Hello from fiber");
            Ctn::Pending(
                (),
                BoxedFiber::new(|| {
                    println!("Hello from continuation");
                    Ctn::Done(())
                }),
            )
        });

        let mut state = fibr.run();
        println!("Interlude");
        if let FiberState::Pending(_, fibr) = state {
            state = fibr.run();
        }
        if let FiberState::Done(_) = state {
            println!("Done.");
        }
    }
}
