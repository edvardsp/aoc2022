use std::collections::HashSet;

struct Input {
    rucksacks: Vec<&'static [u8]>,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let rucksacks = s.split('\n').map(str::as_bytes).collect();
        Self { rucksacks }
    }
}

fn priority(item: u8) -> usize {
    let pri = match item {
        b'a'..=b'z' => item - 97 + 1,
        b'A'..=b'Z' => item - 65 + 27,
        _ => unreachable!(),
    };
    pri as usize
}

fn part1(input: &Input) -> usize {
    input
        .rucksacks
        .iter()
        .map(|&rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            let item = first.iter().copied().find(|c| second.contains(c)).unwrap();
            item
        })
        .map(priority)
        .sum()
}

fn part2(input: &Input) -> usize {
    input
        .rucksacks
        .chunks_exact(3)
        .map(|group| {
            assert!(group.len() == 3);
            let one: HashSet<_> = group[0].iter().copied().collect();
            let two: HashSet<_> = group[1].iter().copied().collect();
            let three: HashSet<_> = group[2].iter().copied().collect();
            let badge = *one.intersection(&two).find(|c| three.contains(c)).unwrap();
            badge
        })
        .map(priority)
        .sum()
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
