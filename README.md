# aramid 🧵

[![Test](https://github.com/mira-merkell/aramid/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/mira-merkell/aramid/actions/workflows/test.yml)
[![Docs](https://github.com/mira-merkell/aramid/actions/workflows/docs.yml/badge.svg?branch=main)](https://github.com/mira-merkell/aramid/actions/workflows/docs.yml)

Synthetic fibers and Rust.

- _very much_ WIP 🚧
- [Fibers][wikipedia-fibers] are little state machines that behave like stackful
  coroutines: when spun, they yield and yield, and then they return. In the
  meantime, they carry their stack around with them.
- Fibers are a model of concurrent computation. They are simple, lightweight and
  well-suited for cooperative multitasking.

## Getting started

- To try the library out, simply add it to your project's dependencies, by
  running the following Cargo command in your project directory:

  ```sh
  cargo add aramid
  ```

  Or add the following line to your Cargo.toml:

  ```toml
  aramid = "0.2.3"
  ```

- The documentation is available online at
  [docs.rs](https://docs.rs/aramid/latest/aramid/).

[wikipedia-fibers]: https://en.wikipedia.org/wiki/Fiber_(computer_science)
[milestone-presentable]: https://github.com/mira-merkell/aramid/milestone/1
