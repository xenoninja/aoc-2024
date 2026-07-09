use std::collections::HashMap;

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

fn part1(input: &str) -> u32 {
    let lines = parse(input);

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in lines {
        let a: Vec<u32> = line
            .split_whitespace()
            .map(|a| a.parse().expect("not number"))
            .collect();
        left_list.push(a[0]);
        right_list.push(a[1]);
    }

    left_list.sort();
    right_list.sort();

    let mut res: u32 = 0;
    for i in 0..left_list.len() {
        res += left_list[i].abs_diff(right_list[i]);
    }
    res
}

fn part2(input: &str) -> u32 {
    let lines = parse(input);

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in lines {
        let a: Vec<u32> = line
            .split_whitespace()
            .map(|a| a.parse().expect("not number"))
            .collect();
        left_list.push(a[0]);
        right_list.push(a[1]);
    }

    let mut occurs: HashMap<u32, u32> = HashMap::new();
    for i in right_list {
        *occurs.entry(i).or_insert(0) += 1;
    }

    let mut res: u32 = 0;
    for i in left_list {
        if occurs.contains_key(&i) {
            res += i * occurs[&i];
        }
    }
    res
}
