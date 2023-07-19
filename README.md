# aramid 🧵

[![Test](https://github.com/mira-merkell/aramid/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/mira-merkell/aramid/actions/workflows/test.yml)
[![Docs](https://github.com/mira-merkell/aramid/actions/workflows/docs.yml/badge.svg?branch=main)](https://github.com/mira-merkell/aramid/actions/workflows/docs.yml)

Synthetic fibers and Rust.

- _very much_ WIP 🚧 If you want to use it, please come back in [6
  months][milestone-presentable]. If you'd like to help hacking, welcome aboard!
  💨⛵🌤️
- [Fibers][wikipedia-fibers] are little state machines that behave like stackful
  coroutines: when spun, they yield and yield, and then they return. In the
  meantime, they carry their stack around with them.
- Fibers are a model of concurrent computation. They are simple, lightweight and
  well-suited for cooperative multitasking.
- Fibers can be turned into iterators over their yielded values; and closures
  with their continuations can be [fibers that live on the heap][api-heapjob].

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
- Check out the `./examples` directory, i.e. a cool implementation of [a monster
  patrolling its dungeon][example-monster] 👾🕹️.

## What's the difference between Fibers and Iterators?

Iterators can be [fibers too][api-trait-fiberiterator]! The difference is mainly
in the fact that Fibers have _two_ return types: `Yield` and `Output`. A nice
trick is to make a fiber yield some values, like an iterator, and then create
another fiber as its final output. Weaving fibers like this can turn a bunch of
them into a powerful finite state machine.

[wikipedia-fibers]: https://en.wikipedia.org/wiki/Fiber_(computer_science)
[milestone-presentable]: https://github.com/mira-merkell/aramid/milestone/1
[api-heapjob]: https://docs.rs/aramid/latest/aramid/struct.HeapJob.html
[example-monster]: ./examples/monster.rs
[api-trait-fiberiterator]:
  https://docs.rs/aramid/latest/aramid/iterators/trait.FiberIterator.html
