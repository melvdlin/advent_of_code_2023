use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;

use rangemap::RangeSet;
use smallvec::SmallVec;
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

    let tables = tables
        .iter()
        .map(Table::reversed)
        .rev()
        .collect::<SmallVec<[Table; 7]>>();
    let min = (0..).find(|n| {
        if n % 1_000_000 == 0 {
            dbg!(n);
        }
        seeds
            .0
            .contains(&tables.iter().fold(*n, |n, table| table.get(n)))
    });

    let result = min.unwrap();
    println!("{result}");

    Ok(())
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

impl Mapping {
    fn reversed(&self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            len: self.len,
        }
    }

    fn reverse(&mut self) {
        let from = self.from;
        self.from = self.to;
        self.to = from;
    }
}

impl From<&Mapping> for Range<i64> {
    fn from(value: &Mapping) -> Self {
        value.from..value.from + value.len
    }
}

#[derive(Debug)]
struct Table(Vec<Mapping>);

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

    fn reversed(&self) -> Self {
        Self(
            self.0
                .iter()
                .map(Mapping::reversed)
                .sorted_unstable_by_key(|mapping| mapping.from)
                .collect_vec(),
        )
    }

    fn reverse(&mut self) {
        self.0.iter_mut().for_each(Mapping::reverse);
        self.0.sort_unstable_by_key(|mapping| mapping.from);
    }
}
