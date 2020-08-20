# Rust sketchbook

This is my sketchbook to learn [Rust](https://www.rust-lang.org/ja/). I learn from the following books.

- [Rust in Action (from Manning)](https://www.manning.com/books/rust-in-action)
  - [source code](https://github.com/rust-in-action/code)
  - very awesome.

## Install tools

```sh
$ cargo install cargo-edit
$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
$ cargo install miniserve
$ cargo install cargo-generate
```

You can use `cargo add` subcommand by installing the `cargo-edit`. Please check [here](https://github.com/killercup/cargo-edit#cargo-add) and [here](https://github.com/rust-lang/cargo/issues/2179#issuecomment-429337378).

## Useful examples

- [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html)
