# Advent of Code

Solutions for Advent of Code, using `cargo-aoc` as helper.

## Preparing a new year

    cargo new --lib --name advent-of-code-2021 2021

Then prepare `src/lib.rs`:

```rust
extern crate aoc_runner_derive;

pub mod day01;

aoc_lib! { year = 2021 }
```

And you're done!

Template for `dayNN.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert!(true);
    }

    #[test]
    fn part2_examples() {
        assert!(true);
    }
}
```
