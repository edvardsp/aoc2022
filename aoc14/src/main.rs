use ndarray::{s, Array2};
use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
struct Coord(usize, usize);

impl From<&str> for Coord {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Coord(x.parse().unwrap(), y.parse().unwrap())
    }
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Air,
    Rock,
    Sand,
}

impl Tile {
    fn as_char(self) -> char {
        match self {
            Tile::Air => '.',
            Tile::Rock => '#',
            Tile::Sand => 'o',
        }
    }
}

#[derive(Clone, Debug)]
struct Map {
    tiles: Array2<Tile>,
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let coords: Vec<Vec<Coord>> = s
            .lines()
            .map(|line| line.split(" -> ").map(Coord::from).collect())
            .collect();

        let max_x = coords
            .iter()
            .flat_map(|line| line.iter())
            .map(|coord| coord.0)
            .max_by(|a, b| a.cmp(b))
            .unwrap();
        let max_y = coords
            .iter()
            .flat_map(|line| line.iter())
            .map(|coord| coord.1)
            .max_by(|a, b| a.cmp(b))
            .unwrap();

        let max_x = 2 * max_x;
        let max_y = max_y + 2;

        let mut tiles = Array2::from_elem([max_y + 1, max_x + 1], Tile::Air);
        for formation in coords.into_iter() {
            for line in formation.windows(2) {
                let start = line[0];
                let end = line[1];
                let (x0, x1) = (start.0.min(end.0), start.0.max(end.0));
                let (y0, y1) = (start.1.min(end.1), start.1.max(end.1));
                tiles.slice_mut(s![y0..=y1, x0..=x1]).fill(Tile::Rock);
            }
        }
        tiles.slice_mut(s![max_y, ..]).fill(Tile::Rock);

        Self { tiles }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.rows() {
            let row: String = row.iter().map(|tile| tile.as_char()).collect();
            write!(f, "{}\n", row)?;
        }
        Ok(())
    }
}

impl Map {
    fn drop_sand(&mut self, x: usize, stop_early: bool) -> Option<()> {
        let mut sand = Coord(x, 0);
        if let Tile::Sand = self.tiles[(sand.1, sand.0)] {
            return None;
        }

        'outer: loop {
            let check = &[
                (sand.1 + 1, sand.0),
                (sand.1 + 1, sand.0 - 1),
                (sand.1 + 1, sand.0 + 1),
            ];
            for &ind in check {
                if let Tile::Air = self.tiles.get(ind)? {
                    sand.0 = ind.1;
                    sand.1 = ind.0;
                    continue 'outer;
                }
            }

            break;
        }

        if stop_early {
            let bedrock = self.tiles.shape()[0] - 2;
            if sand.1 == bedrock {
                return None;
            }
        }

        self.tiles[(sand.1, sand.0)] = Tile::Sand;
        Some(())
    }
}

struct Input {
    map: Map,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        Self { map: Map::from(s) }
    }
}

fn part1(input: &Input) -> impl Display {
    const SOURCE: usize = 500;
    let mut map = input.map.clone();
    for i in 0.. {
        if map.drop_sand(SOURCE, true).is_none() {
            return i;
        }
        // println!("{}", map);
    }
    unreachable!()
}

fn part2(input: &Input) -> impl Display {
    const SOURCE: usize = 500;
    let mut map = input.map.clone();
    for i in 0.. {
        if map.drop_sand(SOURCE, false).is_none() {
            return i;
        }
        // println!("{}", map);
    }
    unreachable!()
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
