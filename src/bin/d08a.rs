use advent_of_code_2023::{load_input, DynResult};
use std::collections::BTreeMap;
use std::env;

const ERR: &str = "parse error";

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;
    let mut input = input.lines();
    let directions = input.next().ok_or(ERR)?;
    input.next();
    let map = input
        .map(|line| {
            let (key, value) = line.split_once('=').ok_or(ERR)?;
            let (_, value) = value.split_once('(').ok_or(ERR)?;
            let (left, right) = value.split_once(',').ok_or(ERR)?;
            let (right, _) = right.split_once(')').ok_or(ERR)?;

            Ok((key.trim(), (left.trim(), right.trim())))
        })
        .collect::<Result<BTreeMap<_, _>, &str>>()?;

    let mut position = "AAA";
    let mut result = 0;
    for (step, direction) in directions.trim().chars().cycle().enumerate() {
        if position == "ZZZ" {
            result = step;
            break;
        }
        position = if direction == 'L' {
            map[position].0
        } else {
            map[position].1
        };
    }

    println!("{result}");
    Ok(())
}
