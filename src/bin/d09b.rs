use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use std::env;

const ERR: &str = "parse error";

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()?;

    let result: i64 = sequences
        .iter()
        .map(|sequence| backstrapolate(sequence))
        .sum::<Option<i64>>()
        .unwrap_or(0);

    println!("{result}");
    Ok(())
}

fn extrapolate(sequence: &[i64]) -> Option<i64> {
    if sequence.is_empty() {
        return None;
    }
    let mut subsequences: Vec<Vec<i64>> = Vec::with_capacity(sequence.len());
    let mut last_sequence = sequence;

    while !last_sequence.iter().all(|value| *value == 0) {
        let next = next_sequence(last_sequence);
        subsequences.push(next);
        last_sequence = subsequences.last().unwrap()
    }

    std::iter::once(sequence.last())
        .chain(subsequences.iter().map(|sequence| sequence.last()))
        .sum()
}

fn backstrapolate(sequence: &[i64]) -> Option<i64> {
    if sequence.is_empty() {
        return None;
    }
    let mut subsequences: Vec<Vec<i64>> = Vec::with_capacity(sequence.len());
    let mut last_sequence = sequence;

    while !last_sequence.iter().all(|value| *value == 0) {
        let next = next_sequence(last_sequence);
        subsequences.push(next);
        last_sequence = subsequences.last().unwrap()
    }

    std::iter::once(sequence.first())
        .chain(subsequences.iter().map(|sequence| sequence.first()))
        .rfold(Some(0), |acc, n| {
            if let (Some(acc), Some(n)) = (acc, n) {
                Some(n - acc)
            } else {
                None
            }
        })
}

fn next_sequence(sequence: &[i64]) -> Vec<i64> {
    sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect_vec()
}
