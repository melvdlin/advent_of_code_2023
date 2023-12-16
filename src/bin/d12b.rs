use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::ops::Range;

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let conditions = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let springs = std::iter::repeat(
                split.next().ok_or("cannot parse springs")?.chars().map(
                    |char| match char {
                        | '.' => Ok(Condition::Working),
                        | '#' => Ok(Condition::Damaged),
                        | '?' => Ok(Condition::Unknown),
                        | _ => Err("could not parse spring"),
                    },
                ),
            )
            .take(5)
            .flatten()
            .collect::<Result<Vec<_>, _>>()?;
            let springs = springs
                .iter()
                .batching(SpringSection::from_iter)
                .filter(|section| !section.is_empty())
                .collect_vec();
            let damaged = std::iter::repeat(
                split
                    .next()
                    .ok_or("cannot parse damaged runs")?
                    .split(',')
                    .map(str::parse::<usize>),
            )
            .take(5)
            .flatten()
            .collect::<Result<Vec<_>, _>>()?;

            DynResult::Ok((springs, damaged))
        })
        .collect::<Result<Vec<_>, _>>()?;

    for (idx, (sections, runs)) in conditions.into_iter().enumerate() {
        println!(
            "{idx}:\n[{} ]\n{runs:?}",
            sections
                .iter()
                .map(|section| format!(" {section:?}"))
                .join(",\n ")
        )
    }

    let result: usize = 0;
    // conditions
    // .iter()
    // .enumerate()
    // .map(|(idx, (springs, damaged_runs))| {
    //     let mut cache = HashMap::new();
    //     let fit = cached_fit(springs, damaged_runs, &mut cache);
    //     println!("{idx}: {fit}");
    //     fit
    // })
    // .sum();

    println!("{result}");
    Ok(())
}

fn cached_fit(
    springs: &[Condition],
    damaged_runs: &[usize],
    cache: &mut HashMap<(*const [Condition], *const [usize]), usize>,
) -> usize {
    *cache
        .entry((springs as *const _, damaged_runs as *const _))
        .or_insert(fit(springs, damaged_runs))
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct SpringSection {
    len: usize,
    damaged: Vec<Range<usize>>,
}

impl SpringSection {
    fn from_iter<'a, I: 'a + Iterator<Item = &'a Condition>>(
        iter: &mut I,
    ) -> Option<Self> {
        let mut range_start = None;
        let mut damaged = Vec::new();
        let mut len = 0;
        let mut not_empty = false;
        for (idx, condition) in iter.enumerate() {
            not_empty = true;
            match condition {
                | Condition::Working => {
                    len = idx;
                    if let Some(start) = range_start {
                        damaged.push(start..idx)
                    }
                    break;
                }
                | Condition::Damaged => {
                    if range_start.is_none() {
                        range_start = Some(idx);
                    }
                }
                | Condition::Unknown => {
                    if let Some(start) = range_start.take() {
                        damaged.push(start..idx);
                    }
                }
            }
        }

        not_empty.then_some(Self { len, damaged })
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
