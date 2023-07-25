//! Various cofunctions and coroutines

/// Cofunction that takes a by-value receiver.
pub trait CoFnOnce<T, R = ()> {
    fn call_once<F>(
        self,
        f: F,
    ) -> R
    where
        F: FnOnce(T) -> R;
}

/// Cofunction that takes a mutable receiver.
pub trait CoFnMut<T, R = ()> {
    fn call_mut<F>(
        &mut self,
        f: F,
    ) -> R
    where
        for<'a> F: FnOnce(&'a mut T) -> R;
}

/// Cofunction that takes an immutable receiver.
pub trait CoFn<T, R = ()> {
    fn call<F>(
        &self,
        f: F,
    ) -> R
    where
        for<'a> F: FnOnce(&'a T) -> R;
}

mod impls {
    use super::{
        CoFn,
        CoFnMut,
    };

    impl<T, R, Co> CoFn<T, R> for &Co
    where
        Co: CoFn<T, R>,
    {
        fn call<F>(
            &self,
            f: F,
        ) -> R
        where
            for<'a> F: FnOnce(&'a T) -> R,
        {
            (**self).call(f)
        }
    }

    impl<T, R, Co> CoFn<T, R> for &mut Co
    where
        Co: CoFn<T, R>,
    {
        fn call<F>(
            &self,
            f: F,
        ) -> R
        where
            for<'a> F: FnOnce(&'a T) -> R,
        {
            (**self).call(f)
        }
    }

    impl<T, R, Co> CoFnMut<T, R> for &mut Co
    where
        Co: CoFnMut<T, R>,
    {
        fn call_mut<F>(
            &mut self,
            f: F,
        ) -> R
        where
            for<'a> F: FnOnce(&'a mut T) -> R,
        {
            (**self).call_mut(f)
        }
    }
}

/// Evaluate functions at an argument.
///
/// # Examples
///
/// ```rust
/// # use aramid::{Eval, CoFn};
/// let coro = Eval::from(2);
/// let f = |x: &_| x * 2;
/// let g = |x: &_| x * 3;
///
/// assert_eq!(coro.call(f), 4);
/// assert_eq!(coro.call(g), 6);
/// ```
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
