use itertools::Itertools;
    use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Display;

type Node = [u8; 2];

struct Input {
    valves: HashMap<Node, usize>,
    graph: HashMap<Node, Vec<Node>>,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let mut valves = HashMap::new();
        let mut graph = HashMap::new();
        for line in s.lines() {
            let tokens: Vec<_> = line.split(' ').collect();
            let node: Node = tokens[1].as_bytes().try_into().unwrap();
            let flow_rate = tokens[4]
                .strip_prefix("rate=")
                .unwrap()
                .strip_suffix(";")
                .unwrap()
                .parse()
                .unwrap();
            let lead_to: Vec<Node> = tokens[9..]
                .iter()
                .copied()
                .map(|c| c.strip_suffix(',').unwrap_or(c))
                .map(|node| node.as_bytes().try_into().unwrap())
                .collect();
            if flow_rate != 0 {
                valves.insert(node, flow_rate);
            }
            graph.insert(node, lead_to);
        }
        Self { valves, graph }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    to: Node,
    cost: usize,
    time_left: usize,
    flow: usize,
    pressure: usize,
    to_open: Vec<Node>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // max heap
        other.time_left.cmp(&self.time_left)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn bfs(graph: &HashMap<Node, Vec<Node>>, vertex: &(Node, Node)) -> usize {
    let mut visited: HashSet<Node> = HashSet::new();
    let mut q: VecDeque<(Node, usize)> = VecDeque::new();
    visited.insert(vertex.0);
    q.push_back((vertex.0, 0));
    while let Some((node, cost)) = q.pop_front() {
        if node == vertex.1 {
            return cost + 1;
        }
        for &next in graph[&node].iter() {
            if visited.insert(next) {
                q.push_back((next, cost + 1));
            }
        }
    }
    unreachable!()
}

fn max_pressure(
    path_cost: &mut HashMap<(Node, Node), usize>,
    valves: &HashMap<Node, usize>,
    graph: &HashMap<Node, Vec<Node>>,
    max_time: usize,
) -> usize {
    const START: Node = *b"AA";
    let to_open: Vec<Node> = valves.keys().copied().collect();

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    for valve in &to_open {
        let vertex = (START, *valve);
        let to_open = to_open
            .iter()
            .copied()
            .filter(|node| node != valve)
            .collect();
        heap.push(State {
            to: *valve,
            cost: bfs(graph, &vertex),
            time_left: max_time,
            flow: 0,
            pressure: 0,
            to_open,
        });
    }

    let mut max_pressure = 0;

    while let Some(State {
        to,
        cost,
        mut time_left,
        mut flow,
        mut pressure,
        to_open,
    }) = heap.pop()
    {
        let cost = cost.min(time_left);
        time_left -= cost;
        pressure += flow * cost;

        if time_left == 0 || to_open.is_empty() {
            max_pressure = max_pressure.max(pressure);
            continue;
        }

        flow += valves[&to];

        for next in to_open.iter() {
            let vertex = (to, *next);
            let cost = *path_cost
                .entry(vertex)
                .or_insert_with(|| bfs(graph, &vertex));

            let to_open = to_open
                .iter()
                .copied()
                .filter(|node| node != next)
                .collect();
            heap.push(State {
                to: *next,
                cost,
                time_left,
                flow,
                pressure,
                to_open,
            });
        }
    }

    max_pressure
}

fn part1(input: &Input) -> impl Display {
    const MAX_TIME: usize = 30;
    let mut path_cost: HashMap<(Node, Node), usize> = HashMap::new();
    max_pressure(&mut path_cost, &input.valves, &input.graph, MAX_TIME)
}

fn part2(input: &Input) -> impl Display {
    const MAX_TIME: usize = 26;
    let mut path_cost: HashMap<(Node, Node), usize> = HashMap::new();
    (0..=input.valves.len() / 2)
        .flat_map(|n| input.valves.clone().into_iter().combinations(n))
        .map(|valves| {
            let valves0: HashMap<Node, usize> = valves.into_iter().collect();
            let valves1: HashMap<Node, usize> = input.valves.clone().into_iter().filter(|(key, _)| !valves0.contains_key(key)).collect();
            max_pressure(&mut path_cost, &valves0, &input.graph, MAX_TIME)
                + max_pressure(&mut path_cost, &valves1, &input.graph, MAX_TIME)
        })
        .max()
        .unwrap()
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
