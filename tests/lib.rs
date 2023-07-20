use aramid::{
    Fiber,
    State,
};

mod iterators;

// struct Cubed(u64, u64);

// impl Cubed {
//     fn new(n: u64) -> Self {
//         Self(n, n)
//     }
// }

// impl Fiber for Cubed {
//     type Output = u64;
//     type Yield = u64;

//     fn run(mut self) -> State<Self> {
//         if self.0 == self.1 {
//             self.1 *= self.1;
//             State::Yield(self)
//         } else {
//             State::Output(self.0 * self.1)
//         }
//     }

//     fn get(&mut self) -> Option<Self::Yield> {
//         Some(self.1)
//     }
// }

// #[test]
// fn squared_01() {
//     let fbr = Cubed::new(3);
//     let state = fbr.run();
//     let mut yld = state.unwrap();
//     assert_eq!(yld.get(), Some(9));

//     let fbr = yld;
//     let state = fbr.run();
//     let out = state.unwrap_done();
//     assert_eq!(out, 27);
// }

// #[test]
// fn squared_iter() {
//     let fbr = Cubed::new(3);
//     let mut res = 0;
//     let collected = fbr
//         .into_iter(|x| {
//             res = x;
//         })
//         .collect::<Vec<_>>();
//     assert_eq!(collected, &[Some(9),]);
//     assert_eq!(res, 27);
// }

// #[test]
// fn squared_iter_try_resume() {
//     let mut res = 0;
//     let mut iter = Cubed::new(3).into_iter(|x| {
//         res = x;
//     });

//     assert_eq!(iter.next(), Some(Some(9)));
//     assert_eq!(iter.next(), None);

//     assert_eq!(iter.next(), None);
//     assert_eq!(res, 27);
// }

// #[test]
// fn squared_complete() {
//     let fbr = Cubed::new(3);
//     let res = fbr.complete(|_| ());
//     assert_eq!(res, 27);
// }

// #[test]
// fn heap_fiber_01() {
//     let fbr = HeapJob::new(|| {
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
