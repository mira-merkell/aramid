# Changelog

## 0.2

### 0.2.3 (??/??/??)

- Add `Iterator <--> Fiber` interface.
- Add `State::complete()`
- API change: 
    - `into_inter()` takes a closure.
    - `State::done_or()` and `::yield_and()` take `&mut` 

### 0.2.2 (18/07/2023)

- Add example: [`monster.rs`](./examples/monster.rs)
- API change: `Fiber::complete()` now calls a closure on every yielded value

### v0.2.1 (18/07/2023)

- Fix bug in FiberIter: 4c7c51b74cdc
- Add GHA tests: 6a5c24276965

### v0.2.0 (18/07/2023)

Initial release
