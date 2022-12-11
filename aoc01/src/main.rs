use std::fmt::Display;

type Elf = Vec<u32>;

#[derive(Clone, Debug)]
struct Input {
    elves: Vec<Elf>,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let elves = s
            .split("\n\n")
            .map(|elf| elf.lines().map(|n| n.parse().unwrap()).collect())
            .collect();
        Self { elves }
    }
}

fn part1(input: &Input) -> impl Display {
    input
        .elves
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        .max()
        .unwrap()
}

fn part2(input: &Input) -> impl Display {
    let mut calories: Vec<u32> = input
        .elves
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        .collect();
    calories.sort_by(|a, b| b.cmp(a));
    calories.into_iter().take(3).sum::<u32>()
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
