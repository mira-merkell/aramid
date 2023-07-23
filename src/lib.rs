//! Synthetic fibers 🧵

/// Lightweight coroutines for cooperative multitasking.
pub trait Fiber {
    /// The type of the yielded values.
    type Yield<'a>
    where
        Self: 'a;
    /// The type of the final output produced by the fiber.
    type Return;

    /// Run the fiber until it yields.
    fn run(&mut self) -> State<Self::Yield<'_>, Self::Return>;
}

/// State of the fiber
#[derive(Debug, PartialEq)]
#[must_use]
pub enum State<Y, R> {
    /// Yielded value
    Yield(Y),
    /// Done processing
    Done(R),
}

impl<Y, R> State<Y, R> {
    /// Unwrap the value wrapped in `Yield`.
    ///
    /// # Panics
    ///
    /// This function panics, if the state is `Done`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use aramid::State;
    /// let state = State::<u8, ()>::Yield(9);
    ///
    /// assert_eq!(state.unwrap(), 9);
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
    /// # use aramid::State;
    /// let state = State::<(), u8>::Done(9);
    ///
    /// assert_eq!(state.unwrap_done(), 9);
    /// ```
    pub fn unwrap_done(self) -> R {
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
    /// # use aramid::State;
    /// let state = State::<u8, ()>::Yield(9);
    ///
    /// assert!(state.is_yield());
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
    /// # use aramid::State;
    /// let state = State::<(), u8>::Done(9);
    ///
    /// assert!(state.is_done());
    /// ```
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
    pub fn done_or<OP>(
        &mut self,
        f: OP,
    ) -> Option<&mut R>
    where
        OP: FnOnce(&mut Y),
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
    pub fn yield_and<OP, T>(
        &mut self,
        f: OP,
    ) -> Option<T>
    where
        OP: FnOnce(&mut Y) -> T,
    {
        if let Self::Yield(fbr) = self {
            Some(f(fbr))
        } else {
            None
        }
    }
}

pub struct HeapJob<'a, T> {
    f: Box<dyn FnMut() -> State<HeapJob<'a, T>, T> + 'a>,
}

impl<'a, T> HeapJob<'a, T> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut() -> State<Self, T> + 'a,
    {
        Self {
            f: Box::new(f)
        }
    }
}

impl<'a, T> Fiber for HeapJob<'a, T> {
    type Return = T;
    type Yield<'any> = () where Self: 'any;

    fn run(&mut self) -> State<Self::Yield<'_>, Self::Return> {
        match (self.f)() {
            State::Yield(yld) => {
                self.f = yld.f;
                State::Yield(())
            }
            State::Done(res) => State::Done(res),
        }
    }
}

pub fn continue_with<'a, F, T>(f: F) -> State<HeapJob<'a, T>, T>
where
    F: FnMut() -> State<HeapJob<'a, T>, T> + 'a,
{
    State::Yield(HeapJob::new(f))
}

#[cfg(test)]
mod tests {
    use State::Done;

    use super::*;

    #[test]
    fn heapjob_01() {
        let mut fbr = HeapJob::new(|| Done(55.5));
        assert_eq!(fbr.run(), Done(55.5));
    }

    #[test]
    fn heapjob_02() {
        let mut fbr = HeapJob::new(|| {
            let res = 55.5;
            continue_with(move || Done(res))
        });
        assert_eq!(fbr.run(), State::Yield(()));
        assert_eq!(fbr.run(), Done(55.5));
    }

    #[test]
    fn heapjob_03() {
        let mut fbr = HeapJob::new(|| {
            let res = 11;
            continue_with(move || {
                let res = res * 5;
                continue_with(move || Done(res))
            })
        });
        assert_eq!(fbr.run(), State::Yield(()));
        assert_eq!(fbr.run(), State::Yield(()));
        assert_eq!(fbr.run(), Done(55));
    }

    #[test]
    fn heapjob_04() {
        let continuation = |res| {
            continue_with(move || {
                let res = res * 5;
                continue_with(move || Done(res))
            })
        };

        let mut fbr = HeapJob::new(move || {
            let res = 11;
            continuation(res)
        });

        assert_eq!(fbr.run(), State::Yield(()));
        assert_eq!(fbr.run(), State::Yield(()));
        assert_eq!(fbr.run(), Done(55));
    }
}
