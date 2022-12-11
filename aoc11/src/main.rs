use dyn_clone::DynClone;
use std::fmt::Display;

trait Operation: DynClone {
    fn call(&self, x: u32) -> u32;
}

impl<F> Operation for F
where
    F: Fn(u32) -> u32 + Clone,
{
    fn call(&self, x: u32) -> u32 {
        self(x)
    }
}

struct Monkey {
    items: Vec<u32>,
    operation: Box<dyn Operation>,
    divisible: u32,
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
                ("*", "old") => Box::new(|x: u32| x * x),
                ("+", "old") => Box::new(|x: u32| x + x),
                ("*", rhs) => {
                    let rhs: u32 = rhs.parse().unwrap();
                    Box::new(move |x: u32| x * rhs)
                }
                ("+", rhs) => {
                    let rhs: u32 = rhs.parse().unwrap();
                    Box::new(move |x: u32| x + rhs)
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
    fn work(&mut self) -> ((usize, Vec<u32>), (usize, Vec<u32>)) {
        self.num_inspections += self.items.len();
        let mut true_items = Vec::new();
        let mut false_items = Vec::new();
        for item in &self.items {
            let worry = self.operation.call(*item);
            if worry % self.divisible == 0 {
                true_items.push(worry / self.divisible);
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

    fn give(&mut self, items: &[u32]) {
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

fn part1(input: &Input) -> impl Display {
    let mut monkeys = input.monkeys.clone();
    for _round in 0..20 {
        for i in 0..input.monkeys.len() {
            let ((monkey0, items0), (monkey1, items1)) = monkeys[i].work();

            monkeys[monkey0].give(items0.as_slice());
            monkeys[monkey1].give(items1.as_slice());
        }
        println!(
            "After round {}, the monkeys are holding items with these worry levels:",
            _round
        );
        for (ind, monkey) in monkeys.iter().enumerate() {
            print!("Monkey {}: ", ind);
            for item in &monkey.items {
                print!("{}, ", item);
            }
            print!("\n");
        }
        println!("");
    }

    let mut monkeys: Vec<_> = monkeys
        .into_iter()
        .map(|monkey| monkey.business())
        .collect();
    println!("{:?}", monkeys);
    monkeys.sort_by(|a, b| b.cmp(a));
    monkeys[0] * monkeys[1]
}

fn part2(input: &Input) -> impl Display {
    todo!();
    0
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2:\n{}", part2(&input));
}
