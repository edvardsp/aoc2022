use dyn_clone::DynClone;
use std::fmt::Display;

trait Operation: DynClone {
    fn call(&self, x: usize) -> usize;
}

impl<F> Operation for F
where
    F: Fn(usize) -> usize + Clone,
{
    fn call(&self, x: usize) -> usize {
        self(x)
    }
}

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Operation>,
    divisible: usize,
    true_monkey: usize,
    false_monkey: usize,
    num_inspections: usize,
}

impl Clone for Monkey {
    fn clone(&self) -> Self {
        Monkey {
            items: self.items.clone(),
            operation: dyn_clone::clone_box(&*self.operation),
            ..*self
        }
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut line_iter = s.lines();
        let _header = line_iter.next().unwrap();
        let items = line_iter
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        let operation: Box<dyn Operation> = {
            let tokens: Vec<_> = line_iter
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Operation: new = ")
                .unwrap()
                .split(' ')
                .collect();
            let lhs = tokens[0];
            let op = tokens[1];
            let rhs = tokens[2];
            assert_eq!(lhs, "old");
            match (op, rhs) {
                ("*", "old") => Box::new(|x: usize| x * x),
                ("+", "old") => Box::new(|x: usize| x + x),
                ("*", rhs) => {
                    let rhs: usize = rhs.parse().unwrap();
                    Box::new(move |x: usize| x * rhs)
                }
                ("+", rhs) => {
                    let rhs: usize = rhs.parse().unwrap();
                    Box::new(move |x: usize| x + rhs)
                }
                _ => unreachable!(),
            }
        };
        let divisible = line_iter
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();
        let true_monkey = line_iter
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let false_monkey = line_iter
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        Self {
            items,
            operation,
            divisible,
            true_monkey,
            false_monkey,
            num_inspections: 0,
        }
    }
}

impl Monkey {
    fn work(
        &mut self,
        manage: impl Fn(usize) -> usize,
    ) -> ((usize, Vec<usize>), (usize, Vec<usize>)) {
        self.num_inspections += self.items.len();
        let mut true_items = Vec::new();
        let mut false_items = Vec::new();
        for item in &self.items {
            let worry = manage(self.operation.call(*item));
            if worry % self.divisible == 0 {
                true_items.push(worry);
            } else {
                false_items.push(worry);
            }
        }
        self.items.clear();
        (
            (self.true_monkey, true_items),
            (self.false_monkey, false_items),
        )
    }

    fn give(&mut self, items: &[usize]) {
        self.items.extend(items);
    }

    fn business(&self) -> usize {
        self.num_inspections
    }
}

struct Input {
    monkeys: Vec<Monkey>,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let monkeys: Vec<_> = s.split("\n\n").map(Monkey::from).collect();

        Self { monkeys }
    }
}

fn monkey_business(monkeys: &Vec<Monkey>, rounds: usize, manage: impl Fn(usize) -> usize) -> usize {
    let mut monkeys = monkeys.clone();
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            let ((monkey0, items0), (monkey1, items1)) = monkeys[i].work(&manage);

            monkeys[monkey0].give(items0.as_slice());
            monkeys[monkey1].give(items1.as_slice());
        }
    }

    let mut monkeys: Vec<_> = monkeys
        .into_iter()
        .map(|monkey| monkey.business())
        .collect();
    println!("{:?}", monkeys);
    monkeys.sort_by(|a, b| b.cmp(a));
    monkeys[0] * monkeys[1]
}

fn part1(input: &Input) -> impl Display {
    monkey_business(&input.monkeys, 20, |x| x / 3)
}

fn part2(input: &Input) -> impl Display {
    let gcd = input
        .monkeys
        .iter()
        .map(|monkey| monkey.divisible)
        .fold(1, |acc, x| acc * x);
    monkey_business(&input.monkeys, 10_000, |x| x % gcd)
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2:\n{}", part2(&input));
}
