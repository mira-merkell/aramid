//! Various cofuntions and coroutines

/// Cofunction that takes an immutable receiver.
pub trait CoFn<T, R = ()>: CoFnMut<T, R> {
    fn call<F>(
        &self,
        f: F,
    ) -> R
    where
        for<'a> F: FnOnce(&'a T) -> R;
}

/// Cofunction that takes a mutable receiver.
pub trait CoFnMut<T, R = ()>: CoFnOnce<T, R> {
    fn call_mut<F>(
        &mut self,
        f: F,
    ) -> R
    where
        for<'a> F: FnOnce(&'a mut T) -> R;
}

/// Cofunction that takes a by-value receiver.
pub trait CoFnOnce<T, R = ()> {
    fn call_once<F>(
        self,
        f: F,
    ) -> R
    where
        F: FnOnce(T) -> R;
}

#[derive(Debug, PartialEq, Default)]
pub struct Eval<T>(T);

impl<T> Eval<T> {
    pub fn take(self) -> T {
        self.0
    }
}

impl<T> From<T> for Eval<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> AsRef<T> for Eval<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Eval<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, R> CoFnOnce<T, R> for Eval<T> {
    fn call_once<F>(
        self,
        f: F,
    ) -> R
    where
        F: FnOnce(T) -> R,
    {
        f(self.0)
    }
}

impl<T, R> CoFnMut<T, R> for Eval<T> {
    fn call_mut<F>(
        &mut self,
        f: F,
    ) -> R
    where
        for<'a> F: FnOnce(&'a mut T) -> R,
    {
        f(&mut self.0)
    }
}

impl<T, R> CoFn<T, R> for Eval<T> {
    fn call<F>(
        &self,
        f: F,
    ) -> R
    where
        for<'a> F: FnOnce(&'a T) -> R,
    {
        f(&self.0)
    }
}
