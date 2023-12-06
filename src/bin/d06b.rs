use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use std::env;

const PARSE_ERR: &str = "parse error";

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let (times, distances) = input.split_once("\n").ok_or(PARSE_ERR)?;
    let time = times
        .split_once(":")
        .ok_or(PARSE_ERR)?
        .1
        .split_whitespace()
        .join("")
        .parse::<usize>()?;
    let distance = distances
        .split_once(":")
        .ok_or(PARSE_ERR)?
        .1
        .split_whitespace()
        .join("")
        .parse::<usize>()?;

    let lower = (0..time).find(|t| t * (time - t) > distance);
    let upper = (0..time).rev().find(|t| t * (time - t) > distance);

    let result = if let (Some(lower), Some(upper)) = (lower, upper) {
        1 + upper - lower
    } else {
        0
    };

    println!("{result}");
    Ok(())
}
