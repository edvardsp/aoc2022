use ndarray::{s, Array, Array2};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize, // Number of steps
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // min heap
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn print_map(map: &Array2<char>) {
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    let shape = map.shape();
    for col in 0..shape[0] {
        let row: String = map.slice(s![col, ..]).iter().collect();
        println!("{}", row);
    }
    std::thread::sleep(std::time::Duration::from_millis(10));
}

fn shortest_path(map: &Array2<u8>, start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    let shape = map.shape();
    let mut dist: Array2<usize> = Array2::from_elem((shape[0], shape[1]), usize::MAX);
    // let mut visited: Array2<char> = Array2::from_elem((shape[0], shape[1]), '.');

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    // visited[start] = '#';
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        // visited[position] = '#';
        // print_map(&visited);
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        let curr_elevation = map[position];
        let adjs = &[
            (position.0.saturating_sub(1), position.1),
            ((position.0 + 1).min(shape[0] - 1), position.1),
            (position.0, position.1.saturating_sub(1)),
            (position.0, (position.1 + 1).min(shape[1] - 1)),
        ];
        for &adj_position in adjs {
            if adj_position == position {
                continue;
            }

            let next_elevation = map[adj_position];
            if next_elevation.saturating_sub(curr_elevation) > 1 {
                continue;
            }

            let next = State {
                cost: cost + 1,
                position: adj_position,
            };
            if next.cost < dist[next.position] {
                // println!(
                //     "Found better cost for {:?} => {:?}",
                //     next.position, next.cost
                // );
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
        // println!("{:?}\n", dist);
    }

    None
}

struct Input {
    map: Array2<u8>,
    start: (usize, usize),
    goal: (usize, usize),
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let values: Vec<_> = s
            .lines()
            .flat_map(|line| line.as_bytes())
            .copied()
            .map(|b| match b {
                b'S' => 100,
                b'E' => 200,
                b'a'..=b'z' => b - b'a',
                _ => unreachable!(),
            })
            .collect();
        let width = s.lines().nth(0).unwrap().len();
        let height = values.len() / width;
        let mut map = Array::from_shape_vec((height, width), values).unwrap();
        let start = map
            .indexed_iter()
            .find(|(_, &value)| value == 100)
            .map(|(ind, _)| ind)
            .unwrap();
        let goal = map
            .indexed_iter()
            .find(|(_, &value)| value == 200)
            .map(|(ind, _)| ind)
            .unwrap();

        map[start] = b'a' - b'a';
        map[goal] = b'z' - b'a';

        Self { map, start, goal }
    }
}

fn part1(input: &Input) -> impl Display {
    shortest_path(&input.map, input.start, input.goal).unwrap()
}

fn part2(input: &Input) -> impl Display {
    input
        .map
        .indexed_iter()
        .filter(|(_, &val)| val == 0)
        .filter_map(|(ind, _)| shortest_path(&input.map, ind, input.goal))
        .min()
        .unwrap()
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
