use super::{
    CoFn,
    CoFnMut,
};

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
