# Changelog

## 0.2

### 0.2.3 (20/07/2023)

- Add `Iterator <--> Fiber` interface.
- Add new methods: `State::complete()`, `Fiber::get_unchecked()`
- API breaking changes:
  - `State::advance()` returns Self
  - `State::done_or()` and `::yield_and()` take `&mut`
  - `Fiber::get()` returns `Option<_>`
  - `Fiber::into_inter()` takes a closure.
- Expand documentation and test suite.

### 0.2.2 (18/07/2023)

- Add example: [`monster.rs`](./examples/monster.rs)
- API breaking changes:
  - `Fiber::complete()` now calls a closure on every yielded value

### v0.2.1 (18/07/2023)

- Fix bug in FiberIter: 4c7c51b74cdc
- Add GHA tests: 6a5c24276965

### v0.2.0 (18/07/2023)

Initial release
