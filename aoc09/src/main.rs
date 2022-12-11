use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
enum Step {
    Left,
    Right,
    Up,
    Down,
}

impl From<u8> for Step {
    fn from(s: u8) -> Self {
        match s {
            b'L' => Step::Left,
            b'R' => Step::Right,
            b'U' => Step::Up,
            b'D' => Step::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Input {
    steps: Vec<(Step, i32)>,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let steps: Vec<_> = s
            .lines()
            .map(|v| {
                let (step, len) = v.split_once(' ').unwrap();
                let step = Step::from(step.as_bytes()[0]);
                let len = len.parse().unwrap();
                (step, len)
            })
            .collect();

        Self { steps }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Knot {
    coord: (i32, i32),
}

impl Knot {
    fn new() -> Self {
        Self { coord: (0, 0) }
    }

    fn step(&mut self, step: Step) {
        match step {
            Step::Left => self.coord.0 -= 1,
            Step::Right => self.coord.0 += 1,
            Step::Up => self.coord.1 += 1,
            Step::Down => self.coord.1 -= 1,
        }
    }

    fn follow(&mut self, parent: Knot) {
        let x_diff = parent.coord.0 - self.coord.0;
        let y_diff = parent.coord.1 - self.coord.1;
        match (x_diff.abs(), y_diff.abs()) {
            (0, 0) | (1, 0) | (0, 1) | (1, 1) => {}
            (_, 0) => {
                self.coord.0 += x_diff.signum();
            }
            (0, _) => {
                self.coord.1 += y_diff.signum();
            }
            (_, _) => {
                self.coord.0 += x_diff.signum();
                self.coord.1 += y_diff.signum();
            }
        }
    }
}

fn simulate_knots(steps: &[(Step, i32)], num: usize) -> usize {
    let mut tail_visit: HashSet<Knot> = HashSet::new();
    let mut knots: Vec<Knot> = vec![Knot::new(); num];
    tail_visit.insert(Knot::new());
    for &(step, len) in steps {
        for _ in 0..len {
            knots[0].step(step);
            for i in 1..knots.len() {
                let parent = knots[i - 1];
                knots[i].follow(parent);
            }
            let tail = knots[knots.len() - 1];
            tail_visit.insert(tail);
        }
    }
    tail_visit.len()
}

fn part1(input: &Input) -> usize {
    simulate_knots(input.steps.as_slice(), 2)
}

fn part2(input: &Input) -> usize {
    simulate_knots(input.steps.as_slice(), 10)
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
