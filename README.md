# aramid 🧵

[![Test](https://github.com/mira-merkell/aramid/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/mira-merkell/aramid/actions/workflows/test.yml)
[![Docs](https://github.com/mira-merkell/aramid/actions/workflows/docs.yml/badge.svg?branch=main)](https://github.com/mira-merkell/aramid/actions/workflows/docs.yml)

Synthetic fibers and Rust.

- _very much_ WIP:  If you want to use it, please come back in 6 months. 🚧
  If you'd like to help hacking, welcome aboard! 💨⛵
- [Fibers][wikipedia-fibers] are little state machines that behave like stackful
  coroutines: when spun, they yield and yield, and then they return. In the
  meantime, they carry their stack around with them.
- Fibers are a model of concurrent computation. They are simple, lightweight and
  well-suited for cooperative multitasking.
- Fibers can be turned into iterators over their yielded values; and closures with
  their continuations can be [fibers that live on the heap][api-heapjob].

## Getting started

- The documentation is available on
  [docs.rs](https://docs.rs/aramid/latest/aramid/).
- Check out our cool example of a fiber that models [a monster patrolling its
  dungeon][example-monster] 👾🕹️.

## What's the difference between Fibers and Iterators?

Iterators can be [fibers too]()! The difference is mainly in the fact that
Fibers have _two_ return types: `Yield` and `Output`. A nice trick is to make a
fiber yield some values, like an iterator, and then create another fiber as a
final output. Weaving fibers like this can turn a bunch of them into a
powerful finite state machine.

[wikipedia-fibers]: https://en.wikipedia.org/wiki/Fiber_(computer_science)
[api-heapjob]: https://docs.rs/aramid/latest/aramid/struct.HeapJob.html
[example-monster]: ./examples/monster.rs
