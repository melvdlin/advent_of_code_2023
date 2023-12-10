use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use std::env;

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let maze = input
        .lines()
        .enumerate()
        .map(|(line_idx, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(char_idx, char)| {
                    Some(match char {
                        | '|' => Some([Direction::Up, Direction::Down]),
                        | '-' => Some([Direction::Left, Direction::Right]),
                        | 'L' => Some([Direction::Right, Direction::Up]),
                        | 'J' => Some([Direction::Left, Direction::Up]),
                        | '7' => Some([Direction::Left, Direction::Down]),
                        | 'F' => Some([Direction::Right, Direction::Down]),
                        | 'S' => None,
                        | _ => None?,
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (start_row, start_col) = maze
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .find_map(|(col_idx, node)| {
                    node.and_then(|node| node.is_none().then_some(col_idx))
                })
                .map(|col_idx| (row_idx, col_idx))
        })
        .ok_or("no start node found")?;

    let maze = maze
        .into_iter()
        .map(|row| row.into_iter().map(|node| node.flatten()).collect_vec())
        .collect_vec();

    let [up, down, left, right] = [
        maze[start_row - 1][start_col],
        maze[start_row + 1][start_col],
        maze[start_row][start_col - 1],
        maze[start_row][start_col + 1],
    ];

    let start_coords = [start_row, start_col];
    let left_coords = [start_row, start_col - 1];
    let right_coords = [start_row, start_col + 1];
    let up_coords = [start_row - 1, start_col];
    let down_coords = [start_row + 1, start_col];

    let mut previous_forward = start_coords;
    let mut previous_backward = start_coords;
    let [mut forward, mut backward] = match [
        connects(left, Direction::Right),
        connects(right, Direction::Left),
        connects(up, Direction::Down),
        connects(down, Direction::Up),
    ] {
        | [true, false, true, false] => [left_coords, up_coords],
        | [true, false, false, true] => [left_coords, down_coords],
        | [true, true, false, false] => [left_coords, right_coords],
        | [false, true, false, true] => [right_coords, down_coords],
        | [false, true, true, false] => [right_coords, up_coords],
        | [false, false, true, true] => [up_coords, down_coords],
        | _ => Err("starting point must connect to exactly two nodes")?,
    };

    let mut distance = 1;
    loop {
        let current = maze[forward[0]][forward[1]];
        let next = find_next(current, forward, previous_forward).ok_or("dead end")?;
        if next == backward {
            break;
        }
        previous_forward = forward;
        forward = next;

        let current = maze[backward[0]][backward[1]];
        let next = find_next(current, backward, previous_backward).ok_or("dead end")?;

        distance += 1;

        if next == forward {
            break;
        }
        previous_backward = backward;
        backward = next;
    }

    let result = distance;

    println!("{result}");
    Ok(())
}

fn find_next(
    current_node: Option<[Direction; 2]>,
    current: [usize; 2],
    previous: [usize; 2],
) -> Option<[usize; 2]> {
    // println!();
    // println!("current:  [{}, {}]", current[0], current[1]);
    // println!("previous: [{}, {}]", previous[0], previous[1]);
    Some(
        if current[0] <= previous[0] && connects(current_node, Direction::Up) {
            // println!("going up!");
            [current[0] - 1, current[1]]
        } else if current[0] >= previous[0] && connects(current_node, Direction::Down) {
            // println!("going down!");
            [current[0] + 1, current[1]]
        } else if current[1] <= previous[1] && connects(current_node, Direction::Left) {
            // println!("going left!");
            [current[0], current[1] - 1]
        } else if current[1] >= previous[1] && connects(current_node, Direction::Right) {
            // println!("going right!");
            [current[0], current[1] + 1]
        } else {
            None?
        },
    )
}

fn connects(node: Option<[Direction; 2]>, direction: Direction) -> bool {
    node.is_some_and(|connections| connections.contains(&direction))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
