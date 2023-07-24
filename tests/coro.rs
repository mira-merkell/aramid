pub trait Coro<T, R = ()> {
    fn coro<F>(
        &self,
        f: F,
    ) -> R
    where
        for<'a> F: FnOnce(&'a T) -> R;
}

pub enum State<Y, R> {
    Yield(Y),
    Return(R),
}

impl<Y, R> State<Y, R> {
    pub fn unwrap(self) -> Y {
        match self {
            Self::Yield(yld) => yld,
            _ => panic!(),
        }
    }
}

pub trait Fiber {
    type Yield;
    type Return;
    type Coroutine<'a>: Coro<Self::Yield>
    where
        Self: 'a;

    fn next(&mut self) -> State<Self::Coroutine<'_>, Self::Return>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyCoro<'a, T>(&'a T);

    impl<'a, T, R> Coro<T, R> for MyCoro<'a, T> {
        fn coro<F>(
            &self,
            f: F,
        ) -> R
        where
            for<'any> F: FnOnce(&'any T) -> R,
        {
            f(self.0)
        }
    }

    #[test]
    fn coro_impl_01() {
        let a = MyCoro(&55.5);
        let f = |x: &f64| x / 5.;
        assert_eq!(a.coro(f), 11.1);
    }

    #[test]
    fn coro_impl_02() {
        let a = MyCoro(&55.5);
        let mut res = 0.;
        let f = |x: &f64| {
            res = x / 5.;
        };
        a.coro(f);
        assert_eq!(res, 11.1);
    }

    struct MyFiber<T>(T);

    impl Fiber for MyFiber<u8> {
        type Coroutine<'a> = MyCoro<'a, u8>
        where
            Self: 'a;
        type Return = ();
        type Yield = u8;

        fn next(&mut self) -> State<Self::Coroutine<'_>, Self::Return> {
            if self.0 == 0 {
                State::Return(())
            } else {
                self.0 -= 1;
                let coro = MyCoro(&self.0);
                State::Yield(coro)
            }
        }
    }

    #[test]
    fn my_fiber_01() {
        let mut fbr = MyFiber(3u8);
        let f = |x: &u8| -(*x as i8);
        let f2 = |x: &u8| *x == 0;

        let cof = fbr.next().unwrap();
        assert_eq!(cof.coro(f), -2);
        assert!(!cof.coro(f2));
        let cof2 = fbr.next().unwrap();
        assert_eq!(cof2.coro(f), -1);
    }
}
