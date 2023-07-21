// use aramid::{
//     Fiber,
//     FiberIterator,
//     State,
// };

// struct MockFiber {
//     count:  u64,
//     steps:  u64,
//     result: bool,
// }

// impl Fiber for MockFiber {
//     type Return = bool;
//     type Yield<'a> = &'a u64;

//     fn run(&mut self) -> State<&'_ u64, bool> {
//         if self.count < self.steps {
//             self.count += 1;
//             State::Yield(&self.count)
//         } else {
//             State::Done(self.result)
//         }
//     }
// }

// #[test]
// fn fiber_iterator_ext() {
//     let iter = 0..3;
//     let mut fbr = iter.into_fiber(11.1);

//     assert_eq!(fbr.run().unwrap(), 0);
//     assert_eq!(fbr.run().unwrap(), 1);
//     assert_eq!(fbr.run().unwrap(), 2);

//     let st = fbr.run();
//     assert_eq!(st.unwrap_done(), 11.1);
// }

// #[test]
// fn fiber_iterator_ext_empty() {
//     let iter = 0..0;
//     let mut fbr = iter.into_fiber(11.1);

//     let st = fbr.run();
//     assert_eq!(st.unwrap_done(), 11.1);
// }

// #[test]
// fn fiber_iterator_ext_lazy() {
//     let iter = 0..3;
//     let mut fbr = iter.into_fiber_lazy(|| 77.7);

//     assert_eq!(fbr.run().unwrap(), 0);
//     assert_eq!(fbr.run().unwrap(), 1);
//     assert_eq!(fbr.run().unwrap(), 2);

//     let st = fbr.run();
//     assert_eq!(st.unwrap_done(), 77.7);
// }

// #[test]
// fn fiber_iterator_ext_lazy_empty() {
//     let iter = 0..0;
//     let mut fbr = iter.into_fiber_lazy(|| 11.1);

//     let st = fbr.run();
//     assert_eq!(st.unwrap_done(), 11.1);
// }
