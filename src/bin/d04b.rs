use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use std::collections::BTreeSet;
use std::env;
use std::str::FromStr;

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let mut cards: Vec<(ScratchCard, usize)> = input
        .lines()
        .map(str::parse::<ScratchCard>)
        .map(|card| card.map(|card| (card, 1)))
        .collect::<Result<Vec<_>, &str>>()?;

    for i in 0..cards.len() {
        let (count, winning) = {
            let (card, count) = &cards[i];
            let winning = card.winning.intersection(&card.yours).collect_vec().len();
            (*count, winning)
        };
        let winning_range = (i + 1).min(cards.len())..(i + 1 + winning).min(cards.len());
        for (_, ref mut other_count) in &mut cards[winning_range] {
            *other_count += count;
        }
    }

    let result: usize = cards.iter().map(|(_, count)| *count).sum();

    println!("{result}");

    Ok(())
}

struct ScratchCard {
    winning: BTreeSet<usize>,
    yours: BTreeSet<usize>,
}

impl FromStr for ScratchCard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERR: &str = "could not parse ScratchCard from the given string";
        const HEADER_SEPARATOR: &str = ":";
        const NUMBERS_SEPARATOR: &str = "|";

        let (_, numbers) = s.split_once(HEADER_SEPARATOR).ok_or(ERR)?;
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

        Ok(Self { winning, yours })
    }
}
