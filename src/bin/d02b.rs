use advent_of_code_2023::{load_input, DynResult};
use std::collections::BTreeMap;
use std::env;
use std::fmt::Debug;
use std::str::FromStr;

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let games: Vec<Game> = input
        .lines()
        .map(str::parse::<Game>)
        .collect::<Result<_, _>>()?;

    let result: usize = games
        .into_iter()
        .map(|game| {
            game.draws.into_iter().fold(
                BTreeMap::<String, usize>::new(),
                |mut acc, draw| {
                    for (colour, count) in draw.into_iter() {
                        acc.entry(colour)
                            .and_modify(|present_count| {
                                *present_count = count.max(*present_count);
                            })
                            .or_insert(count);
                    }
                    acc
                },
            )
        })
        .map(|draw| {
            *draw.get("red").unwrap_or(&0)
                * *draw.get("green").unwrap_or(&0)
                * *draw.get("blue").unwrap_or(&0)
        })
        .sum();

    println!("{result}");

    Ok(())
}

type Draw = BTreeMap<String, usize>;

#[derive(Clone, Debug)]
#[allow(unused)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERR: &str = "could not parse Game from the given string";
        const HEADER_SEPARATOR: char = ':';
        const DRAW_SEPARATOR: char = ';';
        const COLOUR_SEPARATOR: char = ',';
        let (header, draws) = s.split_once(HEADER_SEPARATOR).ok_or(ERR)?;
        let id: usize = header
            .split_whitespace()
            .last()
            .map(str::parse)
            .and_then(Result::ok)
            .ok_or(ERR)?;

        let draws = draws
            .split(DRAW_SEPARATOR)
            .map(|draw| {
                draw.split(COLOUR_SEPARATOR)
                    .map(|colour| {
                        let mut split = colour.split_whitespace();
                        let count: usize = split
                            .next()
                            .map(str::parse)
                            .and_then(Result::ok)
                            .ok_or(ERR)?;
                        let colour = split.next().ok_or(ERR)?.to_owned();
                        if split.next().is_some() {
                            return Err(ERR);
                        }
                        Ok((colour, count))
                    })
                    .collect()
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { id, draws })
    }
}
