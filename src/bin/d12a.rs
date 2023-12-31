use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use std::env;

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let conditions = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let springs = split
                .next()
                .ok_or("cannot parse springs")?
                .chars()
                .map(|char| match char {
                    | '.' => Ok(Condition::Working),
                    | '#' => Ok(Condition::Damaged),
                    | '?' => Ok(Condition::Unknown),
                    | _ => Err("could not parse spring"),
                })
                .collect::<Result<Vec<_>, _>>()?;
            let damaged = split
                .next()
                .ok_or("cannot parse damaged runs")?
                .split(',')
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>()?;

            DynResult::Ok((springs, damaged))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let result: usize = conditions
        .iter()
        .enumerate()
        .map(|(idx, (springs, damaged_runs))| {
            let fit = fit(springs, damaged_runs);
            println!("{idx}: {fit}");
            fit
        })
        .sum();

    println!("{result}");
    Ok(())
}

fn fit(springs: &[Condition], damaged_runs: &[usize]) -> usize {
    let result = (|| {
        let run = if let Some(run) = damaged_runs.first() {
            *run
        } else {
            return if springs.contains(&Condition::Damaged) {
                0
            } else {
                1
            };
        };

        let first_non_working = if let Some((idx, _)) = springs
            .iter()
            .enumerate()
            .find(|(_, condition)| **condition != Condition::Working)
        {
            idx
        } else {
            return 0;
        };

        let springs = &springs[first_non_working..];

        let mut result = 0;
        for (window_start, window) in springs.windows(run).enumerate() {
            let first_working = window
                .iter()
                .enumerate()
                .find_map(|(idx, spring)| (*spring == Condition::Working).then_some(idx));
            let first_damaged = window
                .iter()
                .enumerate()
                .find_map(|(idx, spring)| (*spring == Condition::Damaged).then_some(idx));
            if let Some(working) = first_working {
                if first_damaged.is_some_and(|damaged| damaged < working) {
                    break;
                } else {
                    continue;
                }
            }

            if window_start + window.len() < springs.len() {
                if springs.get(window_start + window.len()) != Some(&Condition::Damaged) {
                    result += fit(
                        &springs[window_start + window.len() + 1..],
                        &damaged_runs[1..],
                    );
                }
            } else if damaged_runs.len() == 1 {
                result += 1;
            }

            if window.first() == Some(&Condition::Damaged) {
                break;
            }
        }

        result
    })();

    // println!(
    //     "{} {:?}: {result}",
    //     springs
    //         .iter()
    //         .map(|condition| match condition {
    //             | Condition::Working => '.',
    //             | Condition::Damaged => '#',
    //             | Condition::Unknown => '?',
    //         })
    //         .join(""),
    //     damaged_runs
    // );

    result
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Condition {
    Working,
    Damaged,
    Unknown,
}
