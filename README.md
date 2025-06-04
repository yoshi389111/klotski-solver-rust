# klotski-solver-rust

A simple Klotski puzzle solver implemented in Rust.

This project was created as a learning exercise to explore Rust programming, particularly in areas like:

- Modeling a sliding puzzle game
- Implementing search algorithms (e.g. BFS)
- Practicing ownership, borrowing, and lifetimes
- Writing idiomatic and efficient Rust code

## Features

- Solves the classic 4x5 Klotski puzzle
- Uses breadth-first search for solution finding
- Command-line interface to input custom puzzles
- Output solution path step-by-step

## Installation

```shell
git clone https://github.com/yoshi389111/klotski-solver-rust
cd klotski-solver-rust
cargo build --release
```

> [!NOTE]
> Use release builds whenever possible, as debug builds take longer to explore.

## Usage

```shell
./target/release/klotski [START_IMAGE] [GOAL_MASK]
```

You can change the initial state of the puzzle and the goal position with the following options:

- First argument: `START_IMAGE`
  - Specify the initial state as a 20-digit hexadecimal number.
  - The leading `0x` is optional. You can insert `_` for readability.
  - The large piece must always be specified as `1`.
  - Default: `0x2113_2113_4556_4786_900a`
- Second argument: `GOAL_MASK`
  - Specify the goal position of the large piece as a 20-digit hexadecimal mask value.
  - Default: `0x0000_0000_0000_0ff0_0ff0`

## What is Klotski?

[Klotski](https://en.wikipedia.org/wiki/Klotski) is a sliding block puzzle where the goal is to move a specific large piece to a target location, usually at the bottom center of a 4x5 grid. It is known as "Hakoiri Musume" (箱入り娘) in Japanese.

## Example Runs

### Example 1:

<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/8/84/Hakoiri3.jpg/330px-Hakoiri3.jpg" alt="Hakoiri3.jpg" width="320">

Image: [Hakoiri3.jpg](https://upload.wikimedia.org/wikipedia/commons/8/84/Hakoiri3.jpg) by [Adan](https://ja.wikipedia.org/wiki/%E5%88%A9%E7%94%A8%E8%80%85:Adan),
licensed under [CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/).

```shell
./target/release/klotski 0x2113_2113_4556_4786_900a
```

<details>
<summary>Result: Solved in 81 moves.</summary>

```txt
step 1: Move piece #7: Down
step 2: Move piece #a: Left
step 3: Move piece #6: Down
step 4: Move piece #5: Right
step 5: Move piece #4: Right
step 6: Move piece #9: Up
step 7: Move piece #7: Left
step 8: Move piece #4: Down
step 9: Move piece #5: Left and Left
step 10: Move piece #8: Up and Right
step 11: Move piece #a: Up and Up
step 12: Move piece #4: Right
step 13: Move piece #9: Right and Down
step 14: Move piece #5: Down
step 15: Move piece #a: Left and Left
step 16: Move piece #8: Left and Left
step 17: Move piece #4: Up
step 18: Move piece #6: Up
step 19: Move piece #9: Right and Right
step 20: Move piece #7: Right and Right
step 21: Move piece #5: Down
step 22: Move piece #8: Down and Left
step 23: Move piece #4: Left
step 24: Move piece #6: Left
step 25: Move piece #3: Down and Down
step 26: Move piece #1: Right
step 27: Move piece #2: Right
step 28: Move piece #a: Up and Up
step 29: Move piece #8: Up and Up
step 30: Move piece #4: Left
step 31: Move piece #2: Down and Down
step 32: Move piece #1: Left
step 33: Move piece #3: Up and Up
step 34: Move piece #6: Right
step 35: Move piece #7: Up and Up
step 36: Move piece #9: Left and Up
step 37: Move piece #5: Right and Right
step 38: Move piece #2: Down
step 39: Move piece #4: Down
step 40: Move piece #7: Left and Left
step 41: Move piece #1: Down
step 42: Move piece #a: Right and Right
step 43: Move piece #8: Up and Right
step 44: Move piece #7: Up and Up
step 45: Move piece #4: Up and Up
step 46: Move piece #2: Left
step 47: Move piece #9: Left and Down
step 48: Move piece #1: Down
step 49: Move piece #a: Down and Left
step 50: Move piece #3: Left
step 51: Move piece #6: Up and Up
step 52: Move piece #1: Right
step 53: Move piece #a: Down
step 54: Move piece #8: Down
step 55: Move piece #7: Right
step 56: Move piece #4: Up
step 57: Move piece #2: Up
step 58: Move piece #9: Left
step 59: Move piece #a: Down and Down
step 60: Move piece #1: Left
step 61: Move piece #6: Down and Down
step 62: Move piece #3: Right
step 63: Move piece #7: Right
step 64: Move piece #8: Right
step 65: Move piece #4: Right
step 66: Move piece #2: Up and Up
step 67: Move piece #1: Left
step 68: Move piece #8: Down and Down
step 69: Move piece #7: Down and Down
step 70: Move piece #3: Left
step 71: Move piece #6: Up and Up
step 72: Move piece #8: Right and Up
step 73: Move piece #5: Up
step 74: Move piece #a: Right and Right
step 75: Move piece #9: Right and Right
step 76: Move piece #1: Down
step 77: Move piece #7: Left and Left
step 78: Move piece #8: Left and Left
step 79: Move piece #5: Up
step 80: Move piece #9: Up and Right
step 81: Move piece #1: Right
```

</details>

### Example 2:

<img src="https://upload.wikimedia.org/wikipedia/commons/6/6e/Hakoiri_Shogi.jpg" alt="Hakoiri_Shogi.jpg" width="320">

Image: [Hakoiri_Shogi.jpg](https://upload.wikimedia.org/wikipedia/commons/6/6e/Hakoiri_Shogi.jpg) by [Tamago915](https://commons.wikimedia.org/wiki/User:Tamago915),
licensed under [CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/).

```shell
./target/release/klotski 0x2113_2113_4567_8899_a00b
```

<details>
<summary>Result: Solved in 40 moves.</summary>

```txt
step 1: Move piece #a: Right and Right
step 2: Move piece #8: Down
step 3: Move piece #9: Left and Left
step 4: Move piece #7: Down and Left
step 5: Move piece #3: Down and Down
step 6: Move piece #1: Right
step 7: Move piece #5: Up and Up
step 8: Move piece #6: Left and Up
step 9: Move piece #7: Up and Left
step 10: Move piece #3: Left
step 11: Move piece #b: Up and Up
step 12: Move piece #a: Right and Up
step 13: Move piece #8: Right and Right
step 14: Move piece #9: Down
step 15: Move piece #7: Down and Left
step 16: Move piece #3: Left
step 17: Move piece #b: Left and Down
step 18: Move piece #1: Down
step 19: Move piece #5: Right and Right
step 20: Move piece #6: Up and Right
step 21: Move piece #3: Up and Up
step 22: Move piece #7: Right and Up
step 23: Move piece #9: Up
step 24: Move piece #8: Left and Left
step 25: Move piece #a: Down
step 26: Move piece #b: Down
step 27: Move piece #1: Down
step 28: Move piece #6: Down and Right
step 29: Move piece #3: Right
step 30: Move piece #7: Up and Up
step 31: Move piece #4: Right and Up
step 32: Move piece #9: Up
step 33: Move piece #8: Up
step 34: Move piece #b: Left and Left
step 35: Move piece #a: Left and Left
step 36: Move piece #1: Down
step 37: Move piece #9: Right and Right
step 38: Move piece #8: Up
step 39: Move piece #a: Up and Left
step 40: Move piece #1: Left
```

</details>

### Example 3:

<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/5b/NTPC-SHJH_HuaRongDao_2013-06.jpg/330px-NTPC-SHJH_HuaRongDao_2013-06.jpg" alt="NTPC-SHJH_HuaRongDao_2013-06.jpg" width="320">

Image: [NTPC-SHJH_HuaRongDao_2013-06.jpg](https://upload.wikimedia.org/wikipedia/commons/5/5b/NTPC-SHJH_HuaRongDao_2013-06.jpg) by [Solomon203](https://commons.wikimedia.org/wiki/User:Solomon203),
licensed under [CC BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/).

```shell
./target/release/klotski 0x2113_2113_4556_7896_700a
```

<details>
<summary>Result: Solved in 77 moves.</summary>

```txt
step 1: Move piece #8: Down and Right
step 2: Move piece #7: Right
step 3: Move piece #4: Down and Down
step 4: Move piece #5: Left
step 5: Move piece #9: Up
step 6: Move piece #8: Up
step 7: Move piece #a: Left
step 8: Move piece #6: Down
step 9: Move piece #9: Right
step 10: Move piece #5: Right
step 11: Move piece #2: Down and Down
step 12: Move piece #1: Left
step 13: Move piece #3: Left
step 14: Move piece #9: Up and Up
step 15: Move piece #6: Up and Up
step 16: Move piece #8: Right
step 17: Move piece #a: Right
step 18: Move piece #7: Right
step 19: Move piece #4: Right and Up
step 20: Move piece #2: Down
step 21: Move piece #5: Left
step 22: Move piece #7: Up
step 23: Move piece #a: Left and Left
step 24: Move piece #7: Down
step 25: Move piece #3: Down
step 26: Move piece #9: Left
step 27: Move piece #6: Up
step 28: Move piece #8: Up
step 29: Move piece #7: Right
step 30: Move piece #3: Down and Down
step 31: Move piece #8: Left and Up
step 32: Move piece #5: Right and Right
step 33: Move piece #4: Up and Left
step 34: Move piece #a: Up and Up
step 35: Move piece #2: Right
step 36: Move piece #4: Down and Down
step 37: Move piece #a: Left and Down
step 38: Move piece #5: Left and Left
step 39: Move piece #6: Down
step 40: Move piece #9: Right
step 41: Move piece #8: Up
step 42: Move piece #3: Up and Up
step 43: Move piece #2: Right
step 44: Move piece #a: Right and Down
step 45: Move piece #5: Down
step 46: Move piece #1: Down
step 47: Move piece #8: Left and Left
step 48: Move piece #9: Left and Left
step 49: Move piece #3: Up
step 50: Move piece #2: Up
step 51: Move piece #6: Up
step 52: Move piece #7: Up
step 53: Move piece #a: Right and Right
step 54: Move piece #4: Right and Right
step 55: Move piece #5: Down
step 56: Move piece #1: Down
step 57: Move piece #9: Down and Left
step 58: Move piece #3: Left
step 59: Move piece #2: Up and Up
step 60: Move piece #1: Right
step 61: Move piece #9: Down and Down
step 62: Move piece #8: Down and Down
step 63: Move piece #3: Left
step 64: Move piece #2: Left
step 65: Move piece #6: Left
step 66: Move piece #7: Up and Up
step 67: Move piece #1: Right
step 68: Move piece #9: Right and Up
step 69: Move piece #5: Up
step 70: Move piece #4: Left and Left
step 71: Move piece #a: Left and Left
step 72: Move piece #1: Down
step 73: Move piece #9: Right and Right
step 74: Move piece #8: Right and Right
step 75: Move piece #5: Up
step 76: Move piece #a: Up and Left
step 77: Move piece #1: Left
```

</details>

## Project Goals

This is not meant to be the fastest or most optimized Klotski solver, but rather a clean and readable example for learning Rust.

If you find any issues or have suggestions, feel free to open an issue or PR!

## Tech Stack

- Rust
- `cargo` for package and build management
- `clap` for command-line argument parsing

## License

MIT License

Copyright (c) 2025 SATO, Yoshiyuki
