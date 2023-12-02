use advent_of_code_2023::{load_input, DynResult};
use std::env;

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let result: usize = input
        .lines()
        .map(|line| {
            (
                line.chars()
                    .find(char::is_ascii_digit)
                    .and_then(|c| c.to_digit(10))
                    .unwrap_or(0),
                line.chars()
                    .rfind(char::is_ascii_digit)
                    .and_then(|c| c.to_digit(10))
                    .unwrap_or(0),
            )
        })
        .map(|vals| 10 * vals.0 + vals.1)
        .map(|val| val as usize)
        .sum();

    println!("{result}");

    Ok(())
}
