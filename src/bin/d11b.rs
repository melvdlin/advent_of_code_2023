use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use nalgebra::Vector2;
use std::env;

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let chart = input.lines().collect_vec();
    let chart_width = chart
        .iter()
        .map(|line| line.len())
        .max()
        .ok_or("map is empty")?;

    let mut horizontal_offsets = Vec::<usize>::with_capacity(chart_width);
    let mut vertical_offsets = Vec::<usize>::with_capacity(chart.len());

    let mut offset = 0;
    for col in 0..chart_width {
        if !chart
            .iter()
            .filter_map(|line| line.get(col..col + 1))
            .contains(&"#")
        {
            offset += 1;
        }
        horizontal_offsets.push(offset);
    }

    let mut offset = 0;
    for line in &chart {
        if !line.contains('#') {
            offset += 1;
        }
        vertical_offsets.push(offset);
    }

    let galaxies = chart
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            let horizontal_offsets = &horizontal_offsets;
            let vertical_offsets = &vertical_offsets;
            line.chars().enumerate().filter_map(move |(x, char)| {
                (char == '#').then_some(Vector2::new(
                    (x + horizontal_offsets[x] * 999_999) as f64,
                    (y + vertical_offsets[y] * 999_999) as f64,
                ))
            })
        })
        .collect_vec();

    let mut result = 0;
    for galaxy in 0..galaxies.len() - 1 {
        let here = galaxies[galaxy];
        for other in &galaxies[galaxy..] {
            result += (other - here).lp_norm(1) as usize;
        }
    }

    println!("{result}");
    Ok(())
}
