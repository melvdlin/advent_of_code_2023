use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use nalgebra::Vector2;
use smallvec::SmallVec;
use std::ops::Sub;
use std::{env, iter, mem};

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let input = input.lines().map(|line| line.chars());

    let numbers = Number::from_lines(input.clone());
    let result: usize = Gear::from_lines(input)
        .iter()
        .filter_map(|gear| {
            numbers
                .iter()
                .clone()
                .filter(|number| {
                    (&number.1 - &gear.0)
                        .is_some_and(|dist| dist.x.abs() <= 1 && dist.y.abs() <= 1)
                })
                .map(|number| number.0)
                .collect_tuple()
                .map(|(n, m)| n * m)
        })
        .sum();

    println!("{result}");

    Ok(())
}

struct Entity {
    locations: SmallVec<[Vector2<isize>; 3]>,
}

impl Sub for &Entity {
    type Output = Option<Vector2<isize>>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.locations
            .iter()
            .cartesian_product(&rhs.locations)
            .map(|(x, y)| x - y)
            .min_by_key(|dist| dist.x.abs() + dist.y.abs())
    }
}

struct Number(usize, Entity);
struct Gear(Entity);

impl Number {
    fn new(value: usize, locations: SmallVec<[Vector2<isize>; 3]>) -> Self {
        Self(value, Entity { locations })
    }

    fn from_lines(lines: impl Iterator<Item = impl Iterator<Item = char>>) -> Vec<Self> {
        let mut result = Vec::new();

        for (line_idx, line) in lines.enumerate() {
            let mut locations = SmallVec::new();
            let mut number: Option<usize> = None;

            for (char_idx, char) in line.enumerate() {
                if let Some(digit) = char.to_digit(10) {
                    if let Some(ref mut number) = number {
                        *number *= 10;
                        *number += digit as usize;
                    } else {
                        number = Some(digit as usize);
                    }
                    locations.push(Vector2::new(char_idx as isize, line_idx as isize));
                } else if let Some(number) = number.take() {
                    result.push(Number::new(
                        number,
                        mem::replace(&mut locations, SmallVec::new()),
                    ))
                }
            }

            if let Some(number) = number {
                result.push(Number::new(number, locations))
            }
        }
        result
    }
}

impl Gear {
    fn new(locations: SmallVec<[Vector2<isize>; 3]>) -> Self {
        Self(Entity { locations })
    }

    fn from_lines(lines: impl Iterator<Item = impl Iterator<Item = char>>) -> Vec<Self> {
        lines
            .map(|line| {
                line.enumerate()
                    .filter_map(|(char_idx, char)| char.eq(&'*').then_some(char_idx))
            })
            .enumerate()
            .flat_map(|(line_idx, gears)| gears.zip(iter::once(line_idx).cycle()))
            .map(|(x, y)| {
                Gear::new(SmallVec::from_elem(Vector2::new(x as isize, y as isize), 1))
            })
            .collect_vec()
    }
}
