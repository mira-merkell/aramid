//! Synthetic fibers 🧵
//!
//! Lightweight coroutines for cooperative multitasking.

mod state;
pub use state::State;

mod traits;
pub use traits::{
    CoFn,
    CoFnMut,
    CoFnOnce,
    Fiber,
    FiberMut,
};
