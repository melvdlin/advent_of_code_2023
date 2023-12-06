use advent_of_code_2023::{load_input, DynResult};
use std::env;

const PARSE_ERR: &str = "parse error";

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let (times, distances) = input.split_once("\n").ok_or(PARSE_ERR)?;
    let times = times
        .split_once(":")
        .ok_or(PARSE_ERR)?
        .1
        .split_whitespace()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()?;
    let distances = distances
        .split_once(":")
        .ok_or(PARSE_ERR)?
        .1
        .split_whitespace()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()?;

    let records = times.into_iter().zip(distances);
    let result: usize = records
        .map(|(time, distance)| {
            let lower = if let Some(lower) = (0..time).find(|t| t * (time - t) > distance)
            {
                lower
            } else {
                return 0;
            };
            let upper = if let Some(upper) =
                (0..time).rev().find(|t| t * (time - t) > distance)
            {
                upper
            } else {
                return 0;
            };
            1 + upper - lower
        })
        .product();

    println!("{result}");
    Ok(())
}
