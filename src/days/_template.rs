// Advent of Code 2024 — day template.
//
// HOW TO USE:
//   1. Copy this file to src/days/dayNN.rs  (NN = zero-padded day number)
//   2. Put your real input at      data/dayNN.txt
//      and the puzzle example at   data/dayNN_example.txt
//   3. Register the module in src/days/mod.rs:   pub mod dayNN;
//   4. Add a match arm in src/main.rs:           "dayNN" => days::dayNN::run(part, use_example),
//   5. Run:   cargo run -- dayNN                (both parts, real input)
//             cargo run -- dayNN example        (both parts, small sample)
//             cargo run -- dayNN part1          (real input)
//             cargo run -- dayNN part2
//             cargo run -- dayNN part1 example  (the small sample)
//
// Keep part1/part2 as pure functions of &str — no file I/O, no globals.
// Read &mut state through local variables only. When both parts share
// expensive parsing, parse once in run() and pass the parsed value to both.

use crate::common::{Part, read_input, time_it};

pub fn run(part: Part, use_example: bool) {
    let day = std::path::Path::new(file!())
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("day source file should have a UTF-8 file stem");
    let name = if use_example {
        format!("{day}_example")
    } else {
        day.to_string()
    };
    let input = read_input(&name);

    // If parsing is shared between parts, do it once here:
    // let parsed = parse(&input);

    match part {
        Part::Part1 => time_it("part 1", || part1(&input)),
        Part::Part2 => time_it("part 2", || part2(&input)),
        Part::All => {
            time_it("part 1", || part1(&input));
            time_it("part 2", || part2(&input));
        }
    }
}

/// Parse the raw input into whatever shape the puzzle needs.
/// Keep this a pure function so it's easy to test.
fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(input: &str) -> u64 {
    let lines = parse(input);
    0
}

fn part2(input: &str) -> u64 {
    let lines = parse(input);
    0
}
