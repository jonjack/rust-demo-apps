# Multiple Binaries

This project demonstrates how to scope multiple binaries into a single project.

You can build all binaries at he same time or individually.

```sh
cargo build

cargo build --bin main1

cargo build --bin main2
```

You cannot run them at the same time so you need to be explicit when executing them.

```sh
cargo run --bin main1
```

You can name you binaries in Cargo.toml.

```sh
[package]
name = "multiple_binaries"
version = "0.1.0"
edition = "2021"

[dependencies]

[[bin]]
name = "program1"
path = "src/bin/main1.rs"

[[bin]]
name = "program2"
path = "src/bin/main2.rs"
```

Now you run them using their name.

```sh
cargo run --bin program1
```
