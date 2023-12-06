use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;

use rangemap::RangeSet;
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
    let seeds = seeds.parse::<Seeds>().map_err(|_| SEED_ERR)?;

    let tables = rest
        .map(|chunk| parse_table(chunk, HEADER_SEPARATOR).ok_or("could not parse table"))
        .collect::<Result<SmallVec<[Table; 7]>, _>>()?;

    let result = solve(seeds, &tables).unwrap_or(0);

    println!("{result}");

    Ok(())
}

fn solve(seeds: Seeds, tables: &[Table]) -> Option<i64> {
    tables
        .iter()
        .fold(seeds, tick)
        .0
        .iter()
        .map(|range| range.start)
        .min()
}

fn tick(mut seeds: Seeds, table: &Table) -> Seeds {
    let mut new_seeds = table
        .0
        .iter()
        .flat_map(|mapping: &Mapping| {
            let source_range = mapping.from..mapping.from + mapping.len;
            let offset = mapping.to - mapping.from;

            let seed_source_ranges = seeds
                .0
                .overlapping(&source_range)
                .map(|seed_range: &Range<i64>| {
                    if source_range.contains(&seed_range.start)
                        && source_range.contains(&seed_range.end)
                    {
                        seed_range.clone()
                    } else if source_range.contains(&seed_range.start) {
                        seed_range.start..source_range.end
                    } else {
                        source_range.start..seed_range.end
                    }
                })
                .collect_vec();

            for seed_source_range in &seed_source_ranges {
                seeds.0.remove(seed_source_range.clone());
            }

            seed_source_ranges
                .iter()
                .map(|overlap| overlap.start + offset..overlap.end + offset)
                .collect_vec()
        })
        .collect::<RangeSet<i64>>();
    new_seeds.extend(seeds.0);
    Seeds(new_seeds)
}

fn parse_table(from: &str, header_separator: &str) -> Option<Table> {
    let (_, table) = from.split_once(header_separator)?;
    table.trim().parse::<Table>().ok()
}

#[derive(Clone, Debug)]
struct Seeds(RangeSet<i64>);

impl FromStr for Seeds {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seeds = RangeSet::new();
        for range in s
            .split_whitespace()
            .map(str::parse::<i64>)
            .chunks(2)
            .into_iter()
            .map(Itertools::collect_tuple::<(_, _)>)
        {
            let (start, len) = range.ok_or(())?;
            let start = start.map_err(|_| ())?;
            let len = len.map_err(|_| ())?;
            let range = start..start + len;
            seeds.insert(range);
        }

        Ok(Self(seeds))
    }
}

#[derive(Copy, Clone, Debug)]
struct Mapping {
    from: i64,
    to: i64,
    len: i64,
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

impl From<&Mapping> for Range<i64> {
    fn from(value: &Mapping) -> Self {
        value.from..value.from + value.len
    }
}

#[derive(Debug)]
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
    fn get(&self, index: i64) -> i64 {
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
