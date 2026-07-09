use std::fs;
use std::time::Instant;

#[derive(Clone, Copy)]
pub enum Part {
    Part1,
    Part2,
    All,
}

impl Part {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "part1" => Some(Self::Part1),
            "part2" => Some(Self::Part2),
            "all" => Some(Self::All),
            _ => None,
        }
    }
}

/// Read a puzzle input file. `name` is like "day01" or "day01_example".
/// Looks under `data/` relative to the crate root.
pub fn read_input(name: &str) -> String {
    let path = format!("data/{name}.txt");
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("failed to read {path}"))
}

/// Run a closure on the named input and print its result with a timing.
/// Use this to wrap `part1` / `part2` so every day prints timings uniformly.
pub fn time_it<R: std::fmt::Display>(label: &str, f: impl FnOnce() -> R) {
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    println!("{label}: {result} ({elapsed:?})");
}
