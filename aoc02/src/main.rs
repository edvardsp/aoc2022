use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl From<u8> for Hand {
    fn from(s: u8) -> Self {
        match s {
            b'A' | b'X' => Hand::Rock,
            b'B' | b'Y' => Hand::Paper,
            b'C' | b'Z' => Hand::Scissor,
            _ => unreachable!(),
        }
    }
}

impl Hand {
    fn value(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }

    fn outcome(&self, other: &Hand) -> usize {
        let score = match self {
            Hand::Rock => match other {
                Hand::Rock => 3,
                Hand::Paper => 0,
                Hand::Scissor => 6,
            },
            Hand::Paper => match other {
                Hand::Rock => 6,
                Hand::Paper => 3,
                Hand::Scissor => 0,
            },
            Hand::Scissor => match other {
                Hand::Rock => 0,
                Hand::Paper => 6,
                Hand::Scissor => 3,
            },
        };
        score + self.value()
    }

    fn strategy(&self, other: &Hand) -> usize {
        let hand = match self {
            // Lose
            Hand::Rock => match other {
                Hand::Rock => Hand::Scissor,
                Hand::Paper => Hand::Rock,
                Hand::Scissor => Hand::Paper,
            },
            // Draw
            Hand::Paper => match other {
                Hand::Rock => Hand::Rock,
                Hand::Paper => Hand::Paper,
                Hand::Scissor => Hand::Scissor,
            },
            // Win
            Hand::Scissor => match other {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissor,
                Hand::Scissor => Hand::Rock,
            },
        };
        hand.outcome(other)
    }
}

#[derive(Clone, Debug)]
struct Input {
    strategy: Vec<(Hand, Hand)>,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let strategy = s
            .lines()
            .map(|line| {
                let bytes = line.as_bytes();
                (Hand::from(bytes[0]), Hand::from(bytes[2]))
            })
            .collect();
        Self { strategy }
    }
}

fn part1(input: &Input) -> impl Display {
    input
        .strategy
        .iter()
        .map(|(other, you)| you.outcome(other))
        .sum::<usize>()
}

fn part2(input: &Input) -> impl Display {
    input
        .strategy
        .iter()
        .map(|(other, you)| you.strategy(other))
        .sum::<usize>()
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
