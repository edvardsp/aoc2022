use std::fmt::Write;

#[derive(Copy, Clone, Debug)]
enum Instr {
    Add(i32),
    Noop,
}

impl From<&str> for Instr {
    fn from(s: &str) -> Self {
        if s == "noop" {
            Instr::Noop
        } else {
            let n = s.split(' ').nth(1).unwrap().parse().unwrap();
            Instr::Add(n)
        }
    }
}

#[derive(Debug)]
struct Input {
    instructions: Vec<Instr>,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let instructions: Vec<_> = s.lines().map(Instr::from).collect();

        Self { instructions }
    }
}

struct SignalStrength {
    cycles: usize,
    score: usize,
    check_at: usize,
}

impl SignalStrength {
    fn new() -> Self {
        SignalStrength {
            cycles: 0,
            score: 0,
            check_at: 20,
        }
    }

    fn advance(&mut self, x: i32) {
        self.cycles += 1;
        if self.cycles == self.check_at {
            let score = self.check_at * x as usize;
            self.score += score as usize;
            self.check_at += 40;
        }
    }
}

fn part1(input: &Input) -> usize {
    let mut x: i32 = 1;
    let mut signal_strength = SignalStrength::new();
    for instr in &input.instructions {
        match instr {
            Instr::Noop => {
                signal_strength.advance(x);
            }
            Instr::Add(n) => {
                signal_strength.advance(x);
                signal_strength.advance(x);
                x += n;
            }
        }
    }
    signal_strength.score
}

struct Crt {
    pixels: [[char; 40]; 6],
    cursor: (usize, usize),
}

impl Crt {
    fn new() -> Self {
        Crt {
            pixels: [[' '; 40]; 6],
            cursor: (0, 0),
        }
    }

    fn draw(&mut self, sprite: i32) {
        if (self.cursor.0 as i32 - sprite).abs() <= 1 {
            self.pixels[self.cursor.1][self.cursor.0] = '█';
        }
        self.cursor.0 += 1;
        if self.cursor.0 == 40 {
            self.cursor.0 = 0;
            self.cursor.1 += 1;
        }
    }
}

impl std::fmt::Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (ind, col) in self.pixels.iter().enumerate() {
            if ind != 0 {
                f.write_char('\n')?;
            }
            let row: String = col.iter().collect();
            f.write_str(&row)?;
        }
        Ok(())
    }
}

fn part2(input: &Input) -> Crt {
    let mut crt = Crt::new();
    let mut x: i32 = 1;
    for instr in &input.instructions {
        match instr {
            Instr::Noop => {
                crt.draw(x);
            }
            Instr::Add(n) => {
                crt.draw(x);
                crt.draw(x);
                x += n;
            }
        }
    }
    crt
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2:\n{}", part2(&input));
}
