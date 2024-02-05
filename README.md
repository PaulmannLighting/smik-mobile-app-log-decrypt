# smik-mobile-app-log-decrypt
Decrypt logs of the smik mobile apps

## Installation
You need to have a [Rust toolchain](https://www.rust-lang.org/) installed.

```
$ git clone https://github.com/PaulmannLighting/smik-mobile-app-log-decrypt.git
$ cd smik-mobile-app-log-decrypt
$ cargo build --release
```

The built binary can be found under `target/release/smik-mobile-app-log-decrypt{,.exe}`.

## Contribution guidelines
* Use `cargo fmt`
* Use `cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::cargo`