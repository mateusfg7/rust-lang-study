# Rust-lang-study

## Unique file

_compile:_

```bash
$ rustc hello.rs
```

_generate docs:_

```bash
$ rustdoc hello.rc
```

## Project

_create new project:_

```bash
$ cargo new [project name]
```

_create a library project:_

```bash
$ cargo new [library project name] --lib
```

_compile and run the project:_

```bash
$ cargo run
```

_install package:_

```bash
$ cargo install [package]
```

_create a final build of project:_

```bash
$ cargo build
```

_create a optimazed build of release_

```bash
$ cargo build --release
```
> _build stored on **target/release/**_

_auto run cargo command_

```bash
$ cargo install cargo-watch
$ cargo watch -x [command, e.g: run]
```
