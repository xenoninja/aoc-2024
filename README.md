# Advent of Code 2024

Solutions in Rust.

## Layout

```
data/                  # puzzle inputs (personal inputs gitignored; examples kept)
  day01.txt
  day01_example.txt
src/
  main.rs              # dispatch: cargo run -- <dayNN> [example]
  common.rs            # shared helpers (read_input, time_it)
  days/
    mod.rs
    day01.rs           # one module per day
```

## Usage

Run a day against your real input:

```
cargo run -- day01
```

Run a day against its small provided example (good for checking parse bugs):

```
cargo run -- day01 example
```

## Conventions

- Each day is a module under `src/days/` exposing `pub fn run(use_example: bool)`.
- `part1` / `part2` are pure functions of `&str` — no file I/O inside them.
- Add a new day: create `src/days/dayNN.rs`, register it in `src/days/mod.rs`,
  and add a match arm in `src/main.rs`.
- Personal puzzle inputs (`data/dayNN.txt`) are gitignored. Example inputs
  (`data/dayNN_example.txt`) are committed.
