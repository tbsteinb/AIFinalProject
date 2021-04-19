# Final Project for AI

## Summary
This project implements an alternative to Paladins auto-purchase system. The
goal of the project is to eventually out-preform the default auto-purchase
system, at least for default champion play-styles.

To keep things simple, this project will not support alternative play-styles,
such as damage Grohk or healing Skye. This should not be a problem because the
default auto-purchase system does not handle these play styles either.

I implemented this project in Rust. This is, in part, because I like using Rust,
but also because I can get roughly the same performance in Rust that I would get
in C or C++.

## Build Steps
To build this project, just run `cargo build --release`

To run the project, run `cargo run --release`

To run the test cases, run `cargo test`
