# AoC 2023

Taking my Rust 🦀 learning journey to [Advent of Code](https://adventofcode.com/2023) 😊

I am aiming for getting a working solution first for each part and then optimizing in future.

## Running the solutions

Each day is a seperate cargo crate with different binaries for the two parts.
For example to run part 2 of day 1:

```
cd day-01 && cargo run --bin part2
```

## Testing

Each day will also include a test module inside `src/lib.rs` which can be run with `cargo test`. I am still learning how the testing framework works so I will figure out a way to run tests for specific days or for all days from the root folder eventually.
