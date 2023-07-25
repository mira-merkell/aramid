use aramid::{
    CoFn,
    CoFnMut,
    CoFnOnce,
};

mod call;
mod call_mut;
mod call_once;

struct EvaluateAt<T>(T);

impl<T, R> CoFnOnce<T, R> for EvaluateAt<T> {
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

impl<T, R> CoFnMut<T, R> for EvaluateAt<T> {
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

impl<T, R> CoFn<T, R> for EvaluateAt<T> {
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
