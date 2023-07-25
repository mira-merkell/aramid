//! Synthetic fibers 🧵
//!
//! Lightweight coroutines for cooperative multitasking.

// ⚰️⚰️⚰️
mod cofn;
pub use cofn::{
    CoFn,
    CoFnMut,
    CoFnOnce,
    Eval,
};

mod iter;
pub use iter::{
    FiberIterator,
    IntoFiber,
};

mod state;
pub use state::State;

mod traits;
pub use traits::{
    Fiber,
    FiberMut,
};
