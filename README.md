# aramid 🧵

Synthetic fibers!

The documentation is available on
[docs.rs](https://docs.rs/aramid/latest/aramid/).

- _very much_ WIP 🚧
- Fibers are little state machines that behave like coroutines: when spun, they
  yield and yield, and then they return. In the meantime, they carry their full
  stack around with them.
- Fibers are a model of concurrent computation. They are static, lightweight and
  particularly well-suited for cooperative multitasking.

The API is built around two tied up traits: `Fiber` and `Yield`: a type
implementing `Fiber` must have an associated type that implements Yield whose
associated type, in turn, must be the original type itself. This way, a type
that implements Fiber becomes automatically a state machine: calling
`Fiber::run()` produces `Yield` wrapped in State::Yield that can elongate the
fiber by calling `Yield::fiber()`. When the fiber is finished, the last call to
run will produce State::Done` variant from which the final result can be
extracted.

Each instance of `Yield` can yield an additional value that need not to be the
same as the type of the final output of the fiber.

The enum `State` contains utility methods for processing yielded values, not
unlike the Standard Library’s `Result` or `Option`.

Additionally, sized fibers can be turned into iterators over their yielded
values, and closures that return a special type: `Continuation` can be turned into
fibers that live on the heap, much like standard coroutines in other languages.
