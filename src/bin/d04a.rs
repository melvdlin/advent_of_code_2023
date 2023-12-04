use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use std::collections::BTreeSet;
use std::env;
use std::str::FromStr;

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let result: usize = input
        .lines()
        .flat_map(str::parse::<ScratchCard>)
        .map(|card| card.yours.intersection(&card.winning).collect_vec().len())
        .map(|winning| {
            if winning > 0 {
                2usize.pow(winning as u32 - 1)
            } else {
                0
            }
        })
        .sum();

    println!("{result}");

    Ok(())
}

struct ScratchCard {
    id: usize,
    winning: BTreeSet<usize>,
    yours: BTreeSet<usize>,
}

impl FromStr for ScratchCard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERR: &str = "could not parse ScratchCard from the given string";
        const HEADER_SEPARATOR: &str = ":";
        const NUMBERS_SEPARATOR: &str = "|";

        let (header, numbers) = s.split_once(HEADER_SEPARATOR).ok_or(ERR)?;
        let id: usize = header
            .split_whitespace()
            .last()
            .map(str::parse)
            .and_then(Result::ok)
            .ok_or(ERR)?;

        let (winning, yours) = numbers.split_once(NUMBERS_SEPARATOR).ok_or(ERR)?;
        let winning = winning
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<BTreeSet<usize>, _>>()
            .map_err(|_| ERR)?;
        let yours = yours
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<BTreeSet<usize>, _>>()
            .map_err(|_| ERR)?;

        Ok(Self { id, winning, yours })
    }
}
