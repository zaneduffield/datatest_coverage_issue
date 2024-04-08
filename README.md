This repo is a minimal reproducible case for an [issue](https://github.com/nextest-rs/datatest-stable/issues/20) I have with coverage not being generated when using the datatest_stable crate.


### Requirements

```sh
cargo install llvm-cov
```

### Steps

1. Generate coverage
    ```sh
    # at root of repo
    cargo llvm-cov --html --open
    ```
2. Observe no hits in `to_cover::main` (despite 'Hello, world!' being printed to stdout)
3. Open `datatests/tests/datatest.rs` and swap the `harness` macro with the main function provided
4. Generate coverage again (step 1).
5. Observe hits in `to_cover::main`.


