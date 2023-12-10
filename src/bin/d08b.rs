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
                    traversal.records[candidate].traversal_indices.clone()
                })
                .collect_vec()
        })
        .collect_vec();

    let end_candidate_combinations = end_candidates
        .iter()
        .multi_cartesian_product()
        .collect_vec();

    dbg!(end_candidate_combinations);

    let result: u64 = 0;

    println!("{result}");
    Ok(())
}

#[derive(Debug, Clone)]
struct TraversalRecord {
    direction_indices: BTreeSet<usize>,
    traversal_indices: BTreeSet<usize>,
}

impl TraversalRecord {
    pub fn new() -> Self {
        Self {
            direction_indices: BTreeSet::new(),
            traversal_indices: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Traversal<'a> {
    start: Node<'a>,
    cycle_start: Node<'a>,
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
        for (traversal_idx, (direction_idx, direction)) in
            directions.iter().enumerate().cycle().enumerate()
        {
            let record = records.entry(position).or_insert_with(TraversalRecord::new);
            if record.direction_indices.contains(&direction_idx) {
                cycle_len = traversal_idx - direction_idx;
                break;
            } else {
                record.direction_indices.insert(direction_idx);
                record.traversal_indices.insert(traversal_idx);
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
