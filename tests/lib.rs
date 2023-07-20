use aramid::{
    Fiber,
    State,
};

struct Cubed(u64, u64);

impl Cubed {
    fn new(n: u64) -> Self {
        Self(n, n)
    }
}

impl Fiber for Cubed {
    type Output = u64;
    type Yield<'a> = &'a u64;

    fn run(&mut self) -> State<&'_ u64, u64> {
        if self.0 == self.1 {
            self.1 *= self.1;
            State::Yield(&self.1)
        } else {
            State::Done(self.0 * self.1)
        }
    }
}

#[test]
fn squared_01() {
    let mut fbr = Cubed::new(3);
    let state = fbr.run();
    assert_eq!(state.unwrap(), &9);

    let state = fbr.run();
    let out = state.unwrap_done();
    assert_eq!(out, 27);
}

#[test]
fn squared_complete() {
    let mut fbr = Cubed::new(3);
    let res = fbr.complete(|_| ());
    assert_eq!(res, 27);
}

// #[test]
// fn heap_fiber_01() {
//     let mut fbr = HeapJob::new(|| {
//         println!("Hello from fiber");

//         continue_with(|| {
//             println!("Hello from continuation");
//             State::Output(5)
//         })
//     });

//     let fbr = fbr.run().unwrap();
//     println!("Interlude");
//     let res = fbr.run();
//     assert_eq!(res.unwrap_done(), 5);
// }

// #[test]
// fn heap_fiber_02() {
//     let fbr = HeapJob::new(|| {
//         println!("Hello from fiber");

//         continue_with(|| {
//             println!("Hello from continuation 1");

//             continue_with(|| {
//                 println!("Hello from continuation 2");
//                 State::Output(5)
//             })
//         })
//     });

//     let fbr = fbr.run().unwrap();
//     println!("Interlude 1");

//     let fbr = fbr.run().unwrap();
//     println!("Interlude 2");

//     let res = fbr.run();
//     assert_eq!(res.unwrap_done(), 5);
// }
