use std::cmp::Ordering;
use std::fmt::Display;

struct Input {
    compare: Vec<(Node, Node)>,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let compare: Vec<_> = s
            .split("\n\n")
            .map(|line| {
                let (lhs, rhs) = line.split_once('\n').unwrap();
                (lhs.into(), rhs.into())
            })
            .collect();

        Self { compare }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Node {
    Array(Vec<Node>),
    Number(usize),
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        serde_json::from_str::<serde_json::Value>(s).unwrap().into()
    }
}

impl From<serde_json::Value> for Node {
    fn from(v: serde_json::Value) -> Self {
        match v {
            serde_json::Value::Array(arr) => {
                let nodes = arr.into_iter().map(Node::from).collect();
                Node::Array(nodes)
            }
            serde_json::Value::Number(n) => {
                let n = n.as_u64().unwrap() as usize;
                Node::Number(n)
            }
            _ => unreachable!(),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Node::Number(x), Node::Number(y)) => x.cmp(y),
            (Node::Number(_), Node::Array(_)) => {
                let new_self = Node::Array(vec![self.clone()]);
                new_self.cmp(other)
            }
            (Node::Array(_), Node::Number(_)) => {
                let new_other = Node::Array(vec![other.clone()]);
                self.cmp(&new_other)
            }
            (Node::Array(x), Node::Array(y)) => {
                for (xi, yi) in x.iter().zip(y.iter()) {
                    match xi.cmp(yi) {
                        Ordering::Equal => {}
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                x.len().cmp(&y.len())
            }
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &Input) -> impl Display {
    input
        .compare
        .iter()
        .enumerate()
        .filter_map(|(ind, (lhs, rhs))| {
            if lhs.cmp(rhs).is_lt() {
                Some(ind + 1)
            } else {
                None
            }
        })
        .sum::<usize>()
}

fn part2(input: &Input) -> impl Display {
    let mut packets: Vec<Node> = input
        .compare
        .iter()
        .cloned()
        .flat_map(|(lhs, rhs)| std::iter::once(lhs).chain(std::iter::once(rhs)))
        .collect();

    let dividers: [Node; 2] = ["[[2]]".into(), "[[6]]".into()];
    packets.extend_from_slice(&dividers);
    packets.sort_unstable();

    dividers
        .iter()
        .map(|divider| {
            let pos = packets.iter().position(|node| node == divider).unwrap();
            pos + 1
        })
        .fold(1, |acc, x| acc * x)
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
