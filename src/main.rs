use std::env;

mod common;
mod days;

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: cargo run -- <dayNN> [example]");
        eprintln!("example: cargo run -- day01");
        eprintln!("example: cargo run -- day01 example");
        std::process::exit(1);
    });
    let use_example = env::args().nth(2).map(|s| s == "example").unwrap_or(false);

    match arg.as_str() {
        "day01" => days::day01::run(use_example),
        // "day02" => days::day02::run(use_example),
        _ => eprintln!("unknown day: {arg}"),
    }
}
