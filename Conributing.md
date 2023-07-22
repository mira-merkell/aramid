# Contributing

- The Rust codebase is formatted according to the settings in `./rustfmt.toml`.
  We enable some unstable features of `rustfmt`. To format your patches
  correctly, you will need the nightly version of the Rust compiler. Before
  opening a pull request, please remove lint from the code by running:

  ```sh
  just lint
  ```
