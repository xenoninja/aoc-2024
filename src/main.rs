use std::env;

use common::Part;

mod common;
mod days;

fn main() {
    let mut args = env::args().skip(1);

    let day = args.next().unwrap_or_else(|| usage());
    let mut part = Part::All;
    let mut use_example = false;

    if let Some(arg) = args.next() {
        if arg == "example" {
            use_example = true;
        } else if let Some(parsed) = Part::parse(&arg) {
            part = parsed;
        } else {
            eprintln!("unknown part or option: {arg}");
            usage();
        }
    }

    if let Some(arg) = args.next() {
        if arg == "example" && !use_example {
            use_example = true;
        } else {
            eprintln!("unexpected argument: {arg}");
            usage();
        }
    }

    match day.as_str() {
        "day01" => days::day01::run(part, use_example),
        // "day02" => days::day02::run(part, use_example),
        _ => eprintln!("unknown day: {day}"),
    }
}

fn usage() -> ! {
    eprintln!("usage: cargo run -- <dayNN> [part1|part2|all] [example]");
    eprintln!("example: cargo run -- day01");
    eprintln!("example: cargo run -- day01 example");
    eprintln!("example: cargo run -- day01 part1");
    eprintln!("example: cargo run -- day01 part1 example");
    std::process::exit(1);
}
