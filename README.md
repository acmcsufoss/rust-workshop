# Rust workshop

This is a quick demo of the rust language that demonstrates a few differences
between C++ and Rust, and includes a function that needs to be translated from
C++ to Rust as the meat of the demo.

## Branches
- **`main`**: Includes the C++ program in it's complete state, but with a memory
  error that causes undefined behavior. Also includes the Rust program in a
  state that compiles and has the to-be-done function commented out.

- **`main-with-bug`**: Same as `main`, but this has a version of the Rust
  program with a Borrow Checker bug. Useful for giving a taste of borrow
  semantics.

- **`completed`**: Contains the completed Rust code with lots of comments.

## Usage

> requires a C++ compiler, GNU Make, and `cargo`

- To compile and run the C++ program, use `make run`.
- To compile and run the Rust program, use `cargo run`.

