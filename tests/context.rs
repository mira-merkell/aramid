//! 🁖 🁖

pub trait Fiber {
    type Yield;

    fn run(
        &mut self,
        cx: &mut Context<'_, Self>,
    );
}

#[derive(Debug, PartialEq)]
pub enum State<Y> {
    Yield(Y),
    Done,
}

// pub struct Scope<'scope, 'env: 'scope, F>
// where
//     F: Fiber + ?Sized,
// {
//     cx:    &'scope mut Context<'env, F>,
//     scope: PhantomData<&'scope mut &'scope ()>,
//     env:   PhantomData<&'env mut &'env ()>,
// }

// impl<'scope, 'env: 'scope, F> Scope<'scope, 'env, F>
// where
//     F: Fiber + ?Sized,
// {
//     pub fn new(cx: &'scope mut Context<'env, F>) -> Self {
//         Self {
//             cx,
//             scope: PhantomData,
//             env: PhantomData,
//         }
//     }

//     pub fn spawn<BODY>(
//         &mut self,
//         f: BODY,
//     ) where BODY: FnOnce() -> F::Yield + 'env,
//     {
//         println!("Hello from spawn");
//         self.cx.stack.insert(0, Box::new(f))
//     }
// }

pub struct Context<'a, F>
where
    F: Fiber + ?Sized,
{
    stack: Vec<Box<dyn FnOnce(&mut Self) -> F::Yield + 'a>>,
}

impl<'a, F> Context<'a, F>
where
    F: Fiber,
{
    pub fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }

    pub fn wrap(
        &mut self,
        f: &'a mut F,
    ) {
        f.run(self);
    }

    // pub fn scope<OP>(
    //     &mut self,
    //     f: OP,
    // ) where for<'scope> OP: FnOnce(&'scope mut Scope<'scope, 'a, F>),
    // {
    //     let mut scope = Scope::new(self);
    //     f(&mut scope)
    // }

    pub fn spawn<BODY>(
        &mut self,
        f: BODY,
    ) where
        BODY: FnOnce(&mut Self) -> F::Yield + 'a,
    {
        self.stack.insert(0, Box::new(f))
    }

    pub fn run(&mut self) -> State<F::Yield> {
        if let Some(f) = self.stack.pop() {
            State::Yield(f(self))
        } else {
            State::Done
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct MockFiber {}

    impl Fiber for MockFiber {
        type Yield = u8;

        fn run(
            &mut self,
            cx: &mut Context<'_, Self>,
        ) {
            cx.spawn(|_| {
                println!("Hello from fiber");
                0
            });
        }
    }

    #[test]
    fn fiber_01() {
        let mut fbr = MockFiber {};
        let mut cx = Context::new();
        cx.wrap(&mut fbr);
        println!("{}", cx.stack.len());

        println!("Run context");
        assert_eq!(cx.run(), State::Yield(0));

        assert_eq!(cx.run(), State::Done)
    }
}
