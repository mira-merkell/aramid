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
- Check out the `./examples` directory, e.g. a cool implementation of [a monster
  patrolling its dungeon][example-monster]. 👾🕹️

## Contributing

- The Rust codebase is formatted according to the settings in `./rustfmt.toml`.
  We enable some unstable features of `rustfmt`. To format your patches
  correctly, you will need the nightly version of the Rust compiler. Before
  opening a pull request, please remove lint from the code by running:

  ```sh
  just lint
  ```

- If you don't know Rust or git/GitHub, but you still want to learn something
  new together, take a look at the [Issues][github-issues] section -- perhaps
  there's something there that you can try to contribute to at your own pace. We
  have all the time in the world here! If it still doesn't look very familiar,
  start a chat in the [Discussions][github-discussions], and we'll see if we can
  work something out. 🦀
- If you cannot afford a computer suitable for software development, GitHub has
  this feature now call Codespaces, where you can set up a Linux environment on
  the cloud. Start a chat in the Discussions and we can try to set it up for
  you. 🔌🖥️

## What's the difference between Fibers and Iterators?

Iterators can be [fibers too][api-trait-fiberiterator]! The difference is mainly
in the fact that Fibers have _two_ return types: `Yield` and `Output`. A nice
trick is to make a fiber yield some values, like an iterator, and then create
another fiber as its final output. Weaving fibers like this can turn a bunch of
them into a powerful finite state machine.

## Where's the runtime?

There isn't any. Just as Rust's Futures are inert and must be [polled by a
runtime][future-poll], when they report ready for work, fibers are simply
_lazy_: they need prodding to advance their state. In fact, implementig a
single-threaded fiber executor is as easy as [running in a
loop][*example comming up*].

[wikipedia-fibers]: https://en.wikipedia.org/wiki/Fiber_(computer_science)
[milestone-presentable]: https://github.com/mira-merkell/aramid/milestone/1
[api-heapjob]: https://docs.rs/aramid/latest/aramid/struct.HeapJob.html
[example-monster]: ./examples/monster.rs
[github-issues]: https://github.com/mira-merkell/aramid/issues
[github-discussions]: https://github.com/mira-merkell/aramid/discussions
[api-trait-fiberiterator]:
  https://docs.rs/aramid/latest/aramid/iterators/trait.FiberIterator.html
[future-poll]:
  https://doc.rust-lang.org/std/future/trait.Future.html#the-poll-method
