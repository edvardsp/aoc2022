use std::collections::HashSet;

#[derive(Debug)]
struct Input {
    signal: &'static [u8],
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let signal = s.as_bytes();

        Self { signal }
    }
}

fn part1(input: &Input) -> usize {
    (4..input.signal.len())
        .into_iter()
        .find(|&i| {
            let chunk = &input.signal[i - 4..i];
            let check: HashSet<_> = chunk.iter().copied().collect();
            check.len() == 4
        })
        .unwrap()
}

fn part2(input: &Input) -> usize {
    (14..input.signal.len())
        .into_iter()
        .find(|&i| {
            let chunk = &input.signal[i - 14..i];
            let check: HashSet<_> = chunk.iter().copied().collect();
            check.len() == 14
        })
        .unwrap()
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
