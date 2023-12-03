use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use smallvec::SmallVec;
use std::env;

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let schematic = input.lines().map(|line| line.as_bytes()).collect_vec();

    let mut result = 0;

    for (row_idx, row) in schematic.iter().enumerate() {
        let mut number = 0;
        let mut is_part_number = false;
        for (col_idx, char) in row.iter().enumerate() {
            if let Some(digit) = (*char as char).to_digit(10) {
                number *= 10;
                number += digit;
                if !is_part_number {
                    let up = 0 < row_idx;
                    let down = row_idx + 1 < schematic.len();
                    let left = 0 < col_idx;
                    let right = col_idx + 1 < row.len();

                    let mut part_candidates = SmallVec::<[u8; 8]>::new();
                    if up && left {
                        part_candidates.push(schematic[row_idx - 1][col_idx - 1]);
                    }
                    if up {
                        part_candidates.push(schematic[row_idx - 1][col_idx])
                    }
                    if up && right {
                        part_candidates.push(schematic[row_idx - 1][col_idx + 1])
                    }
                    if left {
                        part_candidates.push(schematic[row_idx][col_idx - 1])
                    }
                    if right {
                        part_candidates.push(schematic[row_idx][col_idx + 1])
                    }
                    if down && left {
                        part_candidates.push(schematic[row_idx + 1][col_idx - 1])
                    }
                    if down {
                        part_candidates.push(schematic[row_idx + 1][col_idx])
                    }
                    if down && right {
                        part_candidates.push(schematic[row_idx + 1][col_idx + 1])
                    }

                    is_part_number =
                        is_part_number || part_candidates.iter().cloned().any(is_part);
                }
            } else {
                if is_part_number {
                    result += number;
                    is_part_number = false;
                }
                number = 0;
            }
        }
        if is_part_number {
            result += number;
        }
    }

    println!("{result}");

    Ok(())
}

fn is_part(char: u8) -> bool {
    !char.is_ascii_digit() && char != b'.'
}
