#[derive(Debug)]
struct Section(u32, u32);

impl From<&str> for Section {
    fn from(s: &str) -> Self {
        let (left, right) = s.split_once('-').unwrap();
        Self(left.parse().unwrap(), right.parse().unwrap())
    }
}

impl Section {
    fn fully_contains(&self, other: &Section) -> bool {
        self.0 <= other.0 && other.1 <= self.1
    }

    fn disjoint(&self, other: &Section) -> bool {
        if self.0 < other.0 {
            self.0 < other.0 && self.1 < other.0
        } else {
            other.0 < self.0 && other.1 < self.0
        }
    }
}

#[derive(Debug)]
struct Input {
    section_pairs: Vec<(Section, Section)>,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let section_pairs = s
            .split('\n')
            .map(|line| {
                let (lhs, rhs) = line.split_once(',').unwrap();
                (lhs.into(), rhs.into())
            })
            .collect();
        Self { section_pairs }
    }
}

fn part1(input: &Input) -> usize {
    input
        .section_pairs
        .iter()
        .filter(|&(lhs, rhs)| lhs.fully_contains(&rhs) || rhs.fully_contains(&lhs))
        .count()
}

fn part2(input: &Input) -> usize {
    input
        .section_pairs
        .iter()
        .filter(|&(lhs, rhs)| !lhs.disjoint(&rhs))
        .count()
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
