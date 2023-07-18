# aramid 🧵

Synthetic fibers!

The documentation is available on
[docs.rs](https://docs.rs/aramid/latest/aramid/).

- _very much_ WIP 🚧
- Fibers are little state machines that behave like coroutines: when `.run()`,
  they yield and yield, and then they return. In the meantime, they carry their
  full stack around with them.
- Fibers are a model of concurrent computation. They are static, lightweight and
  particularly well-suited for cooperative multitasking.
