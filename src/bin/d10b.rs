use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use nalgebra::{ClosedAdd, Scalar, Vector2};
use nalgebra_glm::vec2_to_vec3;
use num_traits::{One, Zero};
use std::collections::HashMap;
use std::env;
use std::ops::{Add, Neg};

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
                                    .get(&position.add(Direction::Left))
                                    .cloned()
                                    .flatten()
                                    .flatten()
                                    .is_some_and(|pipe| pipe.connects(Direction::Right)),
                                field
                                    .get(&position.add(Direction::Right))
                                    .cloned()
                                    .flatten()
                                    .flatten()
                                    .is_some_and(|pipe| pipe.connects(Direction::Left)),
                                field
                                    .get(&position.add(Direction::Up))
                                    .cloned()
                                    .flatten()
                                    .flatten()
                                    .is_some_and(|pipe| pipe.connects(Direction::Down)),
                                field
                                    .get(&position.add(Direction::Down))
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

    let mut traversal = Vec::new();
    let mut maze = HashMap::new();

    let mut current = start;
    let mut current_node = field[&current].ok_or("dead end")?;
    let mut previous_direction: Option<Direction> = None;
    loop {
        let (direction, next_direction) = if let Some(direction) = previous_direction {
            (
                direction,
                if current_node.connects[0] != direction {
                    current_node.connects[0]
                } else {
                    current_node.connects[1]
                },
            )
        } else {
            let direction = -current_node.connects[0];
            (direction, current_node.connects[1])
        };
        let next = current + next_direction;

        traversal.push(current);
        maze.insert(current, DirectedPipe::new(direction, next_direction));

        if next == start {
            break;
        }

        current = next;
        current_node = field[&current].ok_or("dead end")?;
        previous_direction = Some(-next_direction);
    }

    let tiles = (0..x_extent)
        .cartesian_product(0..y_extent)
        .map(|(x, y)| Vector2::new(x, y))
        .filter(|position| !maze.contains_key(position));
    let tiles = tiles.collect_vec();

    let traversal_float = traversal
        .iter()
        .cloned()
        .map(|pipe| Vector2::<f64>::new(pipe.x as f64, pipe.y as f64))
        .collect_vec();
    let tiles_float = tiles
        .iter()
        .cloned()
        .map(|tile| Vector2::<f64>::new(tile.x as f64, tile.y as f64))
        .collect_vec();

    let result = tiles_float
        .into_iter()
        .filter(|tile| {
            let mut integral: f64 = 0.0;

            let mut previous = if let Some(pipe) = traversal_float.last() {
                pipe
            } else {
                return false;
            } - tile;

            for pipe in &traversal_float {
                let pipe = pipe - tile;
                let angle = pipe.angle(&previous);
                let direction = vec2_to_vec3(&pipe)
                    .cross(&vec2_to_vec3(&previous))
                    .z
                    .signum();
                integral += angle * direction;

                previous = pipe;
            }

            integral.abs() > 0.005
        })
        .count();
    println!("{result}");
    Ok(())
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

impl<T: Scalar + Neg<Output = T> + ClosedAdd<Output = T> + One + Zero> Add<Direction>
    for Vector2<T>
{
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        &self + rhs
    }
}

impl<T: Scalar + Neg<Output = T> + ClosedAdd<Output = T> + One + Zero> Add<Direction>
    for &Vector2<T>
{
    type Output = Vector2<T>;

    fn add(self, rhs: Direction) -> Self::Output {
        self + match rhs {
            | Direction::Left => Vector2::new(-T::one(), T::zero()),
            | Direction::Right => Vector2::new(T::one(), T::zero()),
            | Direction::Up => Vector2::new(T::zero(), -T::one()),
            | Direction::Down => Vector2::new(T::zero(), T::one()),
        }
    }
}

impl Neg for Direction {
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
