# aramid 🧵

Synthetic fibers!

The documentation is available on
[docs.rs](https://docs.rs/aramid/latest/aramid/).

- _very much_ WIP 🚧
- Fibers are little state machines that behave like coroutines: when spun, they
  yield and yield, and then they return. In the meantime, they carry their stack
  around with them.
- Fibers are a model of concurrent computation. They are static, lightweight and
  well-suited for cooperative multitasking.

The enum `State` contains utility methods for processing yielded values, not
unlike the Standard Library's `Result` or `Option`.

Additionally, fibers can be turned into iterators over their yielded values; and
closures that return a `State` can be turned into
[fibers that live on the heap](https://docs.rs/aramid/latest/aramid/struct.HeapJob.html).
