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

/// A coroutine-lending iterator.
pub trait Fiber<T, R = ()> {
    type Coro<'a>: CoFn<T, R>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Coro<'_>>;
}

/// A lending iterator producing coroutines that can mutate the iterator's
/// state.
pub trait FiberMut<T, R = ()> {
    type Coro<'a>: CoFnMut<T, R>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Coro<'_>>;
}
