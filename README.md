# Monkey Rust

This is a Rust implementation of the Monkey programming language as presented in Thorsten Ball's [Writing an Intepreter in Go](https://interpreterbook.com/). As this is an early project in my learning journeys for both Rust and Programming Languages I have tried to stick to the overall feel of the implementation presented in the book, translating to Rust quite directly.

## Running the code

The easiest way to run the code is to clone this repo and use cargo:

```
    cargo run
```

This will start a Monkey REPL.

If you want to build a "release" version then you can use:

```
    cargo build --release
```
and then the exectuable will be at `./target/release/monkey-rust`.

## Developing Monkey

The [Monkey language] is pretty small, and is described fully on the [official website of the book](https://interpreterbook.com/#the-monkey-programming-language) this repo is based on.

## Developing the Interpreter

### Running the tests

You can run all the tests with:

```
    cargo test
```

### Code Organisation

The code has a single binary crate defined in `src/main.rs` and a library crate (in `src/libs.rs`) with multiple modules. Again, the overall structure closely mirrors that in the book with the exception being that tests are defined in the modules they test.

At this point the binary crate simply calls out to the `repl` module.

The `main` branch will be up-to-date with however far I have got with the implementation. The end result of each chapter will be on a branch with a name like `interpreter/chapter-X` so you can check it out and see what the code base looked like at certain point.
