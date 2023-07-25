/// Fiber state.
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
    #[inline]
    pub fn unwrap(self) -> Y {
        self.expect("should be Yield")
    }

    /// Unwrap the value wrapped in `Yield`.
    ///
    /// # Panics
    ///
    /// This function panics with a custom message, if the state is `Done`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use aramid::State;
    /// let state = State::<u8, ()>::Yield(9);
    ///
    /// assert_eq!(state.expect("not done yet"), 9);
    /// ```
    #[inline]
    pub fn expect(
        self,
        msg: &str,
    ) -> Y {
        match self {
            State::Yield(yld) => yld,
            State::Done(_) => panic!("{msg}"),
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
    #[inline]
    pub fn unwrap_done(self) -> R {
        self.expect_done("should be Done")
    }

    /// Unwrap the value wrapped in `Done`.
    ///
    /// # Panics
    ///
    /// This function panics with a custom message, if the state is `Yield`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use aramid::State;
    /// let state = State::<(), u8>::Done(9);
    ///
    /// assert_eq!(state.expect_done("should be done by now"), 9);
    /// ```
    #[inline]
    pub fn expect_done(
        self,
        msg: &str,
    ) -> R {
        match self {
            State::Yield(_) => panic!("{msg}"),
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
    /// Returns `None` if the value was `Yield`.
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
