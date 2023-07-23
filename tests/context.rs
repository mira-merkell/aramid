pub trait Fiber {
    type Yield;
    type Return;

    fn run(
        &mut self,
        cx: &mut Context<'_>,
    );
}

pub struct Context<'a> {
    stack: Vec<Box<dyn FnOnce() + 'a>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }

    pub fn wrap<F: Fiber>(
        &mut self,
        f: &mut F,
    ) {
        f.run(self)
    }

    pub fn spawn<F>(
        &mut self,
        f: F,
    ) where
        F: FnOnce() + 'a,
    {
        self.stack.insert(0, Box::new(f))
    }

    pub fn run(&mut self) {
        let f = self.stack.pop().unwrap();
        f()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct MockFiber {}

    impl Fiber for MockFiber {
        type Return = ();
        type Yield = ();

        fn run(
            &mut self,
            cx: &mut Context<'_>,
        ) {
            cx.spawn(|| println!("Hello from fiber 1"));
            cx.spawn(|| println!("Hello from fiber 2"));
        }
    }

    #[test]
    fn fiber_01() {
        let mut cx = Context::new();
        let mut fbr = MockFiber {};

        cx.wrap(&mut fbr);
        println!("Run context");
        cx.run();
        cx.run();
    }
}
