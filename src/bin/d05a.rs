use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use smallvec::SmallVec;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::env;
use std::ops::Range;
use std::str::FromStr;

fn main() -> DynResult<()> {
    const HEADER_SEPARATOR: &str = ":";
    const SEED_ERR: &str = "could not parse seeds";

    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let mut chunks = input.split("\n\n");
    let seeds = chunks.next().ok_or(SEED_ERR)?;
    let rest = chunks;
    let (_, seeds) = seeds.split_once(HEADER_SEPARATOR).ok_or(SEED_ERR)?;
    let seeds = parse_seeds(seeds)
        .ok_or(SEED_ERR)?
        .into_iter()
        .sorted_unstable()
        .collect_vec();

    let tables = rest
        .map(|chunk| parse_table(chunk, HEADER_SEPARATOR).ok_or("could not parse table"))
        .collect::<Result<SmallVec<[Table; 7]>, _>>()?;

    let locations = seeds
        .iter()
        .cloned()
        .map(|seed| {
            tables.iter().fold(seed, |agricultural_object, table| {
                table.get(agricultural_object)
            })
        })
        .collect_vec();

    let result: u64 = locations.iter().cloned().min().unwrap_or(0);

    println!("{result}");

    Ok(())
}

fn parse_table(from: &str, header_separator: &str) -> Option<Table> {
    let (_, table) = from.split_once(header_separator)?;
    table.trim().parse::<Table>().ok()
}

fn parse_seeds(from: &str) -> Option<Vec<u64>> {
    from.split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()
        .ok()
}

#[derive(Copy, Clone)]
struct Mapping {
    from: u64,
    to: u64,
    len: u64,
}

impl PartialEq<Self> for Mapping {
    fn eq(&self, other: &Self) -> bool {
        self.from.eq(&other.from)
    }
}

impl Eq for Mapping {}

impl PartialOrd<Self> for Mapping {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Mapping {
    fn cmp(&self, other: &Self) -> Ordering {
        self.from.cmp(&other.from)
    }
}

impl From<&Mapping> for Range<u64> {
    fn from(value: &Mapping) -> Self {
        value.from..value.from + value.len
    }
}

struct Table(BTreeSet<Mapping>);

impl FromStr for Table {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .flat_map(str::parse)
                    .collect_tuple()
                    .map(|(to, from, len)| Mapping { from, to, len })
                    .ok_or(())
            })
            .collect::<Result<_, _>>()?;
        Ok(Self(entries))
    }
}

impl Table {
    #[allow(clippy::unnecessary_lazy_evaluations)]
    fn get(&self, index: u64) -> u64 {
        self.0
            .iter()
            .find_map(|mapping| {
                Range::from(mapping)
                    .contains(&index)
                    .then(|| mapping.to + (index - mapping.from))
            })
            .unwrap_or(index)
    }
}
