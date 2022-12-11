#[derive(Debug)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let token: Vec<_> = s.split(' ').collect();
        assert!(token.len() == 6);
        Self {
            num: token[1].parse().unwrap(),
            from: token[3].parse::<usize>().unwrap() - 1,
            to: token[5].parse::<usize>().unwrap() - 1,
        }
    }
}

type Stack = Vec<u8>;

#[derive(Debug)]
struct Input {
    stacks: Vec<Stack>,
    moves: Vec<Move>,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let (init, moves) = s.split_once("\n\n").unwrap();

        let mut init_iter = init.split('\n').rev();
        let num_stacks = init_iter.next().unwrap().split_ascii_whitespace().count();
        let stacks = init_iter
            .map(|line| line.as_bytes())
            .fold(vec![Stack::new(); num_stacks], |mut acc, line| {
                for i in 0..num_stacks {
                    let c = line[i * 4 + 1];
                    if c != b' ' {
                        acc[i].push(c);
                    }
                }
                acc
            })
            .into_iter()
            .collect();

        let moves = moves.split('\n').map(Move::from).collect();

        Self { stacks, moves }
    }
}

fn part1(input: &Input) -> String {
    input
        .moves
        .iter()
        .fold(input.stacks.clone(), |mut stacks, m| {
            let from = &mut stacks[m.from];
            let tail: Vec<_> = from.iter().copied().rev().take(m.num).collect();
            from.resize(from.len() - m.num, 0);
            let to = &mut stacks[m.to];
            to.extend(tail);
            stacks
        })
        .into_iter()
        .map(|stack| *stack.last().unwrap() as char)
        .collect()
}

fn part2(input: &Input) -> String {
    input
        .moves
        .iter()
        .fold(input.stacks.clone(), |mut stacks, m| {
            let from = &mut stacks[m.from];
            let tail: Vec<_> = from.iter().copied().skip(from.len() - m.num).collect();
            from.resize(from.len() - m.num, 0);
            let to = &mut stacks[m.to];
            to.extend(tail);
            stacks
        })
        .into_iter()
        .map(|stack| *stack.last().unwrap() as char)
        .collect()
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
