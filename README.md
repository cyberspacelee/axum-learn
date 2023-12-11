## Dev (REPL)
```shell
# Terminal 1 - For server run.
cargo watch -q -c -w src/ -x "run"
```

```shell
# Terminal 2 - For test.
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

## Dev
```shell
# Terminal 1 - For server run.
cargo run
```

```shell
# Terminal 2 - For test.
cargo test quick_dev -- --nocapture
```


- [Rust](https://github.com/FloWi/rust-axum-course/)
