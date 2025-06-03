# klotski-solver-rust

A simple Klotski puzzle solver implemented in Rust.

This project was created as a learning exercise to explore Rust programming, particularly in areas like:

- Modeling a sliding puzzle game
- Implementing search algorithms (e.g. BFS)
- Practicing ownership, borrowing, and lifetimes
- Writing idiomatic and efficient Rust code

## Features

- Solves the classic 4x5 Klotski puzzle
- Uses breadth-first search (BFS) for solution finding
- Command-line interface to input custom puzzles
- Output solution path step-by-step

## Usage

```shell
git clone https://github.com/yoshi389111/klotski-solver-rust
cd klotski-solver-rust
cargo run --release
```

You can modify the puzzle layout or initial state in the source code.

## Goals

This is not meant to be the fastest or most optimized Klotski solver, but rather a clean and readable example for learning Rust.

If you find any issues or have suggestions, feel free to open an issue or PR!

## What is Klotski?

Klotski is a sliding block puzzle where the goal is to move a specific large block to a target location, usually at the bottom center of a 4x5 grid. It is known as "Hakoiri Musume" (箱入り娘) in Japanese.

## Tech Stack

- Rust
- `cargo` for package and build management
- `clap` for command-line argument parsing

## License

MIT License

Copyright (c) 2025 SATO, Yoshiyuki
