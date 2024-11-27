# Rust Advent of Code 2024 Solution 🦀
A Rust implementation for solving the 2024 advent of code problems. This solution also includes benchmarking for all the problems.

## How to run

Clone the repository and cd into it by running:

```bash
git clone <Repository>
cd <Repository>
```

For running project you can run the following command which will run all the solutions in aoc:

```bash
cargo run
```

If you however want to only run a solution for a specific day you can provide the CLI arguments as follows:

```bash
cargo run -- <day>
```

Additionally you can add the `--release` flag to run in release mode to decrease execution time.

## Benchmarking
The repo includes benchmarking for all the solutions and can be run by the following command:

```bash
cargo bench
```

This will produce benchmarking reports which can be found in the `<Repository>/target/criterion` directory.
