# aramid 🧵

Synthetic fibers!

- _very much_ WIP 🚧
- [Fibers][wikipedia-fibers] are little state machines that behave like
  coroutines: when spun, they yield and yield, and then they return. In the
  meantime, they carry their stack around with them.
- Fibers are a model of concurrent computation. They are static, lightweight and
  well-suited for cooperative multitasking.
- Fibers can represent iterators over their yielded values; and closures can be
  [fibers that live on the heap][api-heapjob].

## Getting started

- The documentation is available on
  [docs.rs](https://docs.rs/aramid/latest/aramid/).
- Check out our cool example of a fiber that models [a monster patrolling its
  dungeon][example-monster] 👾🕹️.

## Where's the runtime?

There isn't any.  As Rust's Futures are inert and need to be polled by runtime,
when they report ready for work, fibers are simply *lazy*: they require prodding
to advance their state.  In fact, implementig single-threaded fiber executor 
is as easy as [running in a loop][].


[wikipedia-fibers]: https://en.wikipedia.org/wiki/Fiber_(computer_science)
[api-heapjob]: https://docs.rs/aramid/latest/aramid/struct.HeapJob.html
[example-monster]: ./examples/monster.rs
