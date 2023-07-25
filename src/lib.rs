//! Synthetic fibers 🧵
//!
//! Lightweight coroutines for cooperative multitasking.

mod cofn;
pub use cofn::{
    CoFn,
    CoFnMut,
    CoFnOnce,
    Eval,
};

mod state;
pub use state::State;

mod traits;
pub use traits::{
    Fiber,
    FiberMut,
};
