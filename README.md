# Rust Advent of Code 2024 Solution 🦀

Welcome to the Rust-based solutions for the Advent of Code 2024! This repository contains efficient and idiomatic Rust code to tackle the intriguing puzzles presented during the event. Additionally, we provide benchmarking tools to measure and optimize the performance of each solution.

## Getting Started 🚀

To begin, ensure you have a working Rust installation. If Rust isn't installed on your system, you can follow the instructions on the [official Rust installation page](https://www.rust-lang.org/tools/install). Verify your installation with the following command:

```bash
rustc --version
```
For a seamless Rust development experience, especially if you're using Visual Studio Code, consider installing these extensions:

-  [Rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) for advanced code analysis and smart code completion.
-  [even better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) for TOML syntax highlighting and formatting.

## Running the Solutions ⚙️

To execute the solutions, first clone the repository and navigate into the project directory:

```bash
git clone <Repository-URL>
cd <Repository-Name>
```

Run all the solutions with:

```bash
cargo run
```

To execute a specific day's solution, supply the day number as a CLI argument:

```bash
cargo run -- <day>
```
Valid CLI arguments include:

-  `1 | one` to run the solution for Day 1.
-  `ex | example` to run the example solution.

For optimized performance, append the `--release` flag to run the solutions in release mode:

```bash
cargo run --release -- <day>
```

## Workflow 🛠️

This project is structured to help you focus on solving the puzzles. Each day's challenge has a corresponding skeleton structure that requires implementing the following methods:

```rust
fn new () -> Self;

fn part_one(&self) -> impl std::fmt::Display;

fn part_two(&self) -> impl std::fmt::Display;
```
You'll find the solution templates in the `src/implementation` directory. An `example.rs` is also provided to demonstrate a sample solution.

## Testing 🧪

To ensure your solutions work as expected, you can write tests and run them with:

```bash
cargo test
```

## Benchmarking 💪

Performance matters! This repository includes benchmarks for all solutions. To run the benchmarks and generate reports:

```bash
cargo bench
```

Benchmarking results are located in the `target/criterion` directory of the repository.
