use std::collections::HashSet;
use std::fmt::Display;

type Coord = (i32, i32);

struct Sensor {
    pos: Coord,
    beacon: Coord,
}

impl Sensor {
    fn manhatten_distance(&self) -> i32 {
        (self.pos.0 - self.beacon.0).abs() + (self.pos.1 - self.beacon.1).abs()
    }

    fn covers_y(&self, y: i32) -> Option<Vec<i32>> {
        let distance = self.manhatten_distance();
        let y_diff = (y - self.pos.1).abs();
        if y_diff > distance {
            return None;
        }

        let x_start = self.pos.0 - (distance - y_diff);
        let x_stop = self.pos.0 + (distance - y_diff);
        let positions: Vec<i32> = (x_start..=x_stop).collect();
        Some(positions)
    }

    fn covers(&self, coord: &Coord) -> bool {
        let x_diff = (self.pos.0 - coord.0).abs();
        let y_diff = (self.pos.1 - coord.1).abs();
        (x_diff + y_diff) <= self.manhatten_distance()
    }

    fn surrounding(&self) -> Vec<Coord> {
        let distance = self.manhatten_distance() + 1;
        let mut positions = Vec::with_capacity((distance * 4) as usize);
        for d in 0..=distance {
            let x = d;
            let y = distance - d;
            if y == 0 {
                positions.push((self.pos.0 + x, self.pos.1));
                positions.push((self.pos.0 - x, self.pos.1));
            } else {
                positions.push((self.pos.0 + x, self.pos.1 + y));
                positions.push((self.pos.0 + x, self.pos.1 - y));
                positions.push((self.pos.0 - x, self.pos.1 + y));
                positions.push((self.pos.0 - x, self.pos.1 - y));
            }
        }
        positions
    }
}

impl From<&str> for Sensor {
    fn from(s: &str) -> Self {
        let s = s.strip_prefix("Sensor at ").unwrap();
        let (pos, beacon) = s.split_once(": closest beacon is at ").unwrap();
        let (pos_x, pos_y) = pos.split_once(", ").unwrap();
        let (beacon_x, beacon_y) = beacon.split_once(", ").unwrap();
        let pos_x = pos_x.strip_prefix("x=").unwrap();
        let pos_y = pos_y.strip_prefix("y=").unwrap();
        let beacon_x = beacon_x.strip_prefix("x=").unwrap();
        let beacon_y = beacon_y.strip_prefix("y=").unwrap();
        let pos = (pos_x.parse().unwrap(), pos_y.parse().unwrap());
        let beacon = (beacon_x.parse().unwrap(), beacon_y.parse().unwrap());
        Self { pos, beacon }
    }
}

struct Input {
    sensors: Vec<Sensor>,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let sensors = s.lines().map(Sensor::from).collect();
        Self { sensors }
    }
}

fn part1(input: &Input) -> impl Display {
    const Y: i32 = 2000000;

    let beacons: HashSet<i32> = input
        .sensors
        .iter()
        .filter_map(|sensor| {
            if sensor.beacon.1 == Y {
                Some(sensor.beacon.0)
            } else {
                None
            }
        })
        .collect();

    let mut positions: HashSet<i32> = input
        .sensors
        .iter()
        .filter_map(|sensor| sensor.covers_y(Y))
        .flat_map(|positions| positions.into_iter())
        .collect();
    positions.retain(|x| !beacons.contains(x));

    positions.len()
}

fn part2(input: &Input) -> impl Display {
    const X_MAX: i32 = 4000000;
    const Y_MAX: i32 = 4000000;

    let (x, y) = input
        .sensors
        .iter()
        .flat_map(|sensor| sensor.surrounding().into_iter())
        .filter(|&(x, y)| x >= 0 && x <= X_MAX && y >= 0 && y <= Y_MAX)
        .find(|coord| !input.sensors.iter().any(|sensor| sensor.covers(coord)))
        .unwrap();

    let tuning = x as usize * 4000000 + y as usize;
    tuning
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
