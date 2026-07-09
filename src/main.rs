use std::env;

use common::Part;

mod common;
mod days;

fn main() {
    let mut args = env::args().skip(1);

    let day = args.next().unwrap_or_else(|| usage());
    let part_arg = args.next().unwrap_or_else(|| usage());
    let part = Part::parse(&part_arg).unwrap_or_else(|| {
        eprintln!("unknown part: {part_arg}");
        usage();
    });

    let use_example = match args.next() {
        None => false,
        Some(flag) if flag == "example" => true,
        Some(flag) => {
            eprintln!("unknown option: {flag}");
            usage();
        }
    };

    if let Some(extra) = args.next() {
        eprintln!("unexpected argument: {extra}");
        usage();
    }

    match day.as_str() {
        "day01" => days::day01::run(part, use_example),
        // "day02" => days::day02::run(part, use_example),
        _ => eprintln!("unknown day: {day}"),
    }
}

fn usage() -> ! {
    eprintln!("usage: cargo run -- <dayNN> <part1|part2|all> [example]");
    eprintln!("example: cargo run -- day01 part1");
    eprintln!("example: cargo run -- day01 part2");
    eprintln!("example: cargo run -- day01 all");
    eprintln!("example: cargo run -- day01 part1 example");
    std::process::exit(1);
}
