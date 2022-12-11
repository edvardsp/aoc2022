use ndarray::{s, Array, Array2};

#[derive(Debug)]
struct Input {
    map: Array2<u32>,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let values: Vec<_> = s
            .lines()
            .flat_map(|line| line.as_bytes())
            .map(|&v| (v - b'0') as u32)
            .collect();
        let width = s.lines().nth(0).unwrap().len();
        let height = values.len() / width;
        let map = Array::from_shape_vec((height, width), values).unwrap();

        Self { map }
    }
}

fn part1(input: &Input) -> usize {
    let map = &input.map;
    let shape = map.shape();
    let mut total = shape[0] * 2 + shape[1] * 2 - 4;
    for (ind, &c) in map
        .slice(s![1..shape[0] - 1, 1..shape[1] - 1])
        .indexed_iter()
    {
        let ind = (ind.0 + 1, ind.1 + 1);
        let is_visible = (map
            .slice(s![ind.0, ind.1 + 1..])
            .iter()
            .copied()
            .max()
            .unwrap()
            < c)
            || (map.slice(s![ind.0, ..ind.1]).iter().copied().max().unwrap() < c)
            || (map
                .slice(s![ind.0 + 1.., ind.1])
                .iter()
                .copied()
                .max()
                .unwrap()
                < c)
            || (map.slice(s![..ind.0, ind.1]).iter().copied().max().unwrap() < c);
        if is_visible {
            total += 1;
        }
    }
    total
}

fn see_trees<'a>(view: impl Iterator<Item = &'a u32>, base: u32) -> u32 {
    let mut summa = 0;
    for t in view.copied() {
        summa += 1;
        if t >= base {
            break;
        }
    }
    summa
}

fn part2(input: &Input) -> u32 {
    let map = &input.map;
    let shape = map.shape();
    let mut max_scenic_score = 0;
    for (ind, &c) in map
        .slice(s![1..shape[0] - 1, 1..shape[1] - 1])
        .indexed_iter()
    {
        let ind = (ind.0 + 1, ind.1 + 1);
        let up_view = see_trees(map.slice(s![..ind.0, ind.1]).iter().rev(), c);
        let left_view = see_trees(map.slice(s![ind.0, ..ind.1]).iter().rev(), c);
        let right_view = see_trees(map.slice(s![ind.0, ind.1 + 1..]).iter(), c);
        let down_view = see_trees(map.slice(s![ind.0 + 1.., ind.1]).iter(), c);
        let scenic_score = left_view * right_view * up_view * down_view;
        max_scenic_score = max_scenic_score.max(scenic_score);
    }
    max_scenic_score
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
