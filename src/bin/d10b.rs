use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use nalgebra::{ClosedAdd, Scalar, Vector2};
use num_traits::{One, Zero};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::env;
use std::ops::Neg;

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;

    let field = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(col, char)| {
                    (
                        Vector2::new(col as isize, row as isize),
                        Pipe::from_char(char),
                    )
                })
                .collect_vec()
        })
        .collect::<HashMap<_, _>>();

    let (start, _) = field
        .iter()
        .find(|(_, node)| node.is_some_and(|pipe| pipe.is_none()))
        .ok_or("no start found")?;
    let start = *start;

    let x_extent = field
        .keys()
        .map(|position| position.x)
        .max()
        .ok_or("field is empty")?
        + 1;
    let y_extent = field
        .keys()
        .map(|position| position.y)
        .max()
        .ok_or("field is empty")?
        + 1;

    let field = field
        .iter()
        .map(|(position, node)| {
            (
                *position,
                node.and_then(|pipe| {
                    pipe.or_else(|| {
                        Some(
                            match [
                                field
                                    .get(&walk(*position, Direction::Left))
                                    .cloned()
                                    .flatten()
                                    .flatten()
                                    .is_some_and(|pipe| pipe.connects(Direction::Right)),
                                field
                                    .get(&walk(*position, Direction::Right))
                                    .cloned()
                                    .flatten()
                                    .flatten()
                                    .is_some_and(|pipe| pipe.connects(Direction::Left)),
                                field
                                    .get(&walk(*position, Direction::Up))
                                    .cloned()
                                    .flatten()
                                    .flatten()
                                    .is_some_and(|pipe| pipe.connects(Direction::Down)),
                                field
                                    .get(&walk(*position, Direction::Down))
                                    .cloned()
                                    .flatten()
                                    .flatten()
                                    .is_some_and(|pipe| pipe.connects(Direction::Up)),
                            ] {
                                | [true, true, false, false] => {
                                    Pipe::from([Direction::Left, Direction::Right])
                                }
                                | [true, false, true, false] => {
                                    Pipe::from([Direction::Left, Direction::Up])
                                }
                                | [true, false, false, true] => {
                                    Pipe::from([Direction::Left, Direction::Down])
                                }
                                | [false, true, true, false] => {
                                    Pipe::from([Direction::Right, Direction::Up])
                                }
                                | [false, true, false, true] => {
                                    Pipe::from([Direction::Right, Direction::Down])
                                }
                                | [false, false, true, true] => {
                                    Pipe::from([Direction::Up, Direction::Down])
                                }
                                | _ => None?,
                            },
                        )
                    })
                }),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut maze_traversal = Vec::new();
    let mut maze = HashMap::new();

    let mut current = start;
    let mut current_node = field[&current].ok_or("dead end")?;
    let mut direction = current_node.connects[0];
    loop {
        let next_direction = current_node
            .connects
            .iter()
            .cloned()
            .find(|direction_candidate| *direction_candidate != -direction)
            .ok_or("dead end")?;
        let next = walk(current, next_direction);

        maze_traversal.push(current);
        maze.insert(current, DirectedPipe::new(direction, next_direction));

        if next == start {
            break;
        }

        current = next;
        current_node = field[&current].ok_or("dead end")?;
        direction = next_direction;
    }

    let tiles = (0..x_extent)
        .cartesian_product(0..y_extent)
        .map(|(x, y)| Vector2::new(x, y))
        .filter(|position| !maze.contains_key(position));
    let tiles = tiles.collect_vec();

    dbg!(tiles.len());

    let result = tiles
        .iter()
        .filter(|tile| {
            let mut pro = 0;
            let mut contra = 0;
            let x = tile.x;
            let y = tile.y;
            for x in 0..x {
                if let Some(pipe) = maze.get(&Vector2::new(x, y)) {
                    let directions = pipe.traversal_directions();
                    if directions.contains(&Direction::Up) {
                        if directions.contains(&Direction::Right) {
                            contra += 1;
                        } else {
                            pro += 1;
                        }
                    }
                    if directions.contains(&Direction::Down) {
                        if directions.contains(&Direction::Left) {
                            contra += 1;
                        } else {
                            pro += 1;
                        }
                    }
                }
            }

            if pro != contra {
                return false;
            }

            for x in x + 1..x_extent {
                if let Some(pipe) = maze.get(&Vector2::new(x, y)) {
                    let directions = pipe.traversal_directions();
                    if directions.contains(&Direction::Up) {
                        if directions.contains(&Direction::Right) {
                            contra += 1;
                        } else {
                            pro += 1;
                        }
                    }
                    if directions.contains(&Direction::Down) {
                        if directions.contains(&Direction::Left) {
                            contra += 1;
                        } else {
                            pro += 1;
                        }
                    }
                }
            }

            pro == contra
        })
        .count();

    println!("{result}");
    Ok(())
}

fn walk<T: Scalar + Neg<Output = T> + ClosedAdd<Output = T> + One + Zero>(
    from: Vector2<T>,
    direction: Direction,
) -> Vector2<T> {
    from + match direction {
        | Direction::Left => Vector2::new(-T::one(), T::zero()),
        | Direction::Right => Vector2::new(T::one(), T::zero()),
        | Direction::Up => Vector2::new(T::zero(), -T::one()),
        | Direction::Down => Vector2::new(T::zero(), T::one()),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DirectedPipe {
    from: Direction,
    to: Direction,
}

impl DirectedPipe {
    fn new(from: Direction, to: Direction) -> Self {
        Self { from, to }
    }

    fn traversal_directions(&self) -> SmallVec<[Direction; 2]> {
        let mut result = SmallVec::new();
        result.push(-self.from);
        if self.to != -self.from {
            result.push(self.to);
        }
        result
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Pipe {
    connects: [Direction; 2],
}

impl Pipe {
    fn from_char(char: char) -> Option<Option<Self>> {
        Some(match char {
            | '|' => Some(Self::from([Direction::Up, Direction::Down])),
            | '-' => Some(Self::from([Direction::Left, Direction::Right])),
            | 'L' => Some(Self::from([Direction::Right, Direction::Up])),
            | 'J' => Some(Self::from([Direction::Left, Direction::Up])),
            | '7' => Some(Self::from([Direction::Left, Direction::Down])),
            | 'F' => Some(Self::from([Direction::Right, Direction::Down])),
            | 'S' => None,
            | _ => None?,
        })
    }

    fn connects(self, direction: Direction) -> bool {
        self.connects[0] == direction || self.connects[1] == direction
    }
}

impl From<[Direction; 2]> for Pipe {
    fn from(connects: [Direction; 2]) -> Self {
        Self { connects }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl std::ops::Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            | Self::Left => Self::Right,
            | Self::Right => Self::Left,
            | Self::Up => Self::Down,
            | Self::Down => Self::Up,
        }
    }
}
