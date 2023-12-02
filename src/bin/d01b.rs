use advent_of_code_2023::{load_input, DynResult};
use std::env;

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let result: usize = input.lines().map(mapper).sum();

    println!("{result}");

    Ok(())
}

fn mapper(s: &str) -> usize {
    const DIGITS: [(&str, usize); 20] = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let mut first = None;
    let mut last = None;
    for i in 0..s.len() {
        for (string, value) in DIGITS {
            if first.is_none() && s[i..].starts_with(string) {
                first = Some(value);
            }
            if last.is_none() && s[..s.len() - i].ends_with(string) {
                last = Some(value);
            }
            if first.is_some() && last.is_some() {
                break;
            }
        }
    }

    first.unwrap_or(0) * 10 + last.unwrap_or(0)
}
