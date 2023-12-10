use advent_of_code_2023::{load_input, DynResult};
use itertools::Itertools;
use rayon::iter::*;
use std::collections::{BTreeMap, BTreeSet};
use std::env;

const ERR: &str = "parse error";

fn main() -> DynResult<()> {
    let input = load_input(env::args_os().nth(1).expect("no input provided"))?;
    let mut input = input.lines();
    let directions = input
        .next()
        .ok_or(ERR)?
        .chars()
        .filter_map(|char| match char {
            | 'L' => Some(Direction::Left),
            | 'R' => Some(Direction::Right),
            | _ => None,
        })
        .collect_vec();
    input.next();
    let map = input
        .map(|line| {
            let (key, value) = line.split_once('=').ok_or(ERR)?;
            let (_, value) = value.split_once('(').ok_or(ERR)?;
            let (left, right) = value.split_once(',').ok_or(ERR)?;
            let (right, _) = right.split_once(')').ok_or(ERR)?;
            Ok((
                Node::from(key.trim()),
                (Node::from(left.trim()), Node::from(right.trim())),
            ))
        })
        .collect::<Result<BTreeMap<_, _>, &str>>()?;

    let positions = map
        .keys()
        .filter(|position| position.is_start())
        .cloned()
        .collect_vec();

    let traversals = positions
        .into_par_iter()
        .map(|position| Traversal::new(position, &map, &directions))
        .collect::<Vec<_>>();

    let end_candidates = traversals
        .iter()
        .map(|traversal| {
            traversal
                .end_candidates
                .iter()
                .flat_map(|candidate| {
                    traversal.records[candidate]
                        .traversal_to_direction_indices
                        .keys()
                        .cloned()
                        .clone()
                })
                .collect_vec()
        })
        .collect_vec();

    let end_candidate_combinations = end_candidates
        .iter()
        .map(|vec| vec.iter().cloned())
        .multi_cartesian_product()
        .collect_vec();

    let result: u64 = end_candidate_combinations
        .into_par_iter()
        .filter_map(|combination| {
            if combination.len() < 2 {
                return None;
            }
            let mut positions = combination
                .iter()
                .enumerate()
                .map(|(idx, position)| (*position as u64, &traversals[idx]))
                .sorted_unstable_by_key(|(position, _)| *position)
                .rev()
                .collect_vec();

            // check for end nodes that are outside of the cycle
            for i in 1..positions.len() {
                let position = positions[i].0;
                let traversal = positions[i].1;
                if position < traversal.cycle_start_idx as u64
                    && position != positions[0].0
                {
                    return None;
                }
            }

            // let mut n = 0;
            loop {
                let reference = positions[0].0;
                // if n % 10_000_000 == 0 {
                //     dbg!(n, reference);
                // }
                for (position, traversal) in positions[1..].iter_mut() {
                    while *position < reference {
                        *position += traversal.cycle_len as u64;
                    }
                }
                if positions.iter().map(|(position, _)| position).all_equal() {
                    break;
                } else {
                    positions[0].0 += positions[0].1.cycle_len as u64;
                }
                // n += 1;
            }
            Some(positions[0].0)
        })
        .min()
        .unwrap_or(0);

    println!("{result}");
    Ok(())
}

#[derive(Debug, Clone)]
struct TraversalRecord {
    direction_to_traversal_indices: BTreeMap<usize, usize>,
    traversal_to_direction_indices: BTreeMap<usize, usize>,
}

impl TraversalRecord {
    pub fn new() -> Self {
        Self {
            direction_to_traversal_indices: BTreeMap::new(),
            traversal_to_direction_indices: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Traversal<'a> {
    start: Node<'a>,
    cycle_start: Node<'a>,
    cycle_start_idx: usize,
    records: BTreeMap<Node<'a>, TraversalRecord>,
    end_candidates: BTreeSet<Node<'a>>,
    cycle_len: usize,
}

impl<'a> Traversal<'a> {
    fn new(
        start: Node<'a>,
        graph: &BTreeMap<Node<'a>, (Node<'a>, Node<'a>)>,
        directions: &[Direction],
    ) -> Self {
        let mut records = BTreeMap::new();
        let mut end_candidates = BTreeSet::new();
        let mut position = start;
        let mut cycle_len = 0;
        let mut cycle_start_idx = 0;
        for (traversal_idx, (direction_idx, direction)) in
            directions.iter().enumerate().cycle().enumerate()
        {
            let record = records.entry(position).or_insert_with(TraversalRecord::new);
            if let Some(cycle_start) =
                record.direction_to_traversal_indices.get(&direction_idx)
            {
                cycle_len = traversal_idx - direction_idx;
                cycle_start_idx = *cycle_start;
                break;
            } else {
                record
                    .direction_to_traversal_indices
                    .insert(direction_idx, traversal_idx);
                record
                    .traversal_to_direction_indices
                    .insert(traversal_idx, direction_idx);
                if position.is_end() {
                    end_candidates.insert(position);
                }
            }
            position = if *direction == Direction::Left {
                graph[&position].0
            } else {
                graph[&position].1
            }
        }

        Self {
            start,
            cycle_start: position,
            cycle_start_idx,
            records,
            end_candidates,
            cycle_len,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Node<'a> {
    id: &'a str,
}

impl<'a> Node<'a> {
    fn is_start(&self) -> bool {
        self.id.ends_with('A')
    }

    fn is_end(&self) -> bool {
        self.id.ends_with('Z')
    }
}

impl<'a> From<&'a str> for Node<'a> {
    fn from(value: &'a str) -> Self {
        Self { id: value }
    }
}
