use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug)]
enum Command {
    Cd(&'static str),
    Ls,
}

impl From<&'static str> for Command {
    fn from(s: &'static str) -> Self {
        if s.starts_with("cd") {
            let (_, dir) = s.split_once(' ').unwrap();
            Command::Cd(dir)
        } else if s == "ls" {
            Command::Ls
        } else {
            panic!("Unexpected Command str {}", s);
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Output {
    Dir(&'static str),
    File(&'static str, usize),
}

impl From<&'static str> for Output {
    fn from(s: &'static str) -> Self {
        match s.split_once(' ').unwrap() {
            ("dir", dir) => Output::Dir(dir),
            (size, file) => Output::File(file, size.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
struct Input {
    command_pairs: Vec<(Command, Vec<Output>)>,
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        let mut line_iter = s.split('\n').peekable();
        let mut command_pairs = Vec::new();
        while let Some(command) = line_iter.next() {
            let command: Command = command.strip_prefix("$ ").unwrap().into();
            let mut outputs = Vec::new();
            while let Some(output) = line_iter.next_if(|&line| !line.starts_with('$')) {
                outputs.push(output.into());
            }
            command_pairs.push((command, outputs));
        }

        Self { command_pairs }
    }
}

fn part1(input: &Input) -> usize {
    let mut files: HashSet<PathBuf> = HashSet::new();
    let mut sizes: HashMap<PathBuf, usize> = HashMap::new();
    let mut curr_dir: PathBuf = PathBuf::new();
    for (command, outputs) in &input.command_pairs {
        //println!("{:?} -> {:?}", command, outputs);
        match command {
            Command::Cd(dir) => {
                if *dir == "/" {
                    curr_dir = PathBuf::new();
                    curr_dir.push("/")
                } else if *dir == ".." {
                    curr_dir.pop();
                } else {
                    curr_dir.push(dir);
                }
            }
            Command::Ls => {
                for output in outputs {
                    match output {
                        Output::Dir(_dir) => {
                            //parents.insert(dir, curr_dir);
                        }
                        Output::File(file, size) => {
                            let file = curr_dir.as_path().join(file).to_owned();
                            files.insert(file.clone());
                            sizes.insert(file, *size);
                        }
                    }
                }
            }
        }
    }

    let mut stack: VecDeque<PathBuf> = files.iter().cloned().collect();
    let mut visited: HashSet<PathBuf> = HashSet::new();
    while let Some(path) = stack.pop_front() {
        if !visited.insert(path.clone()) {
            continue;
        }
        let mut parent = path.clone();
        if parent.pop() {
            let size = *sizes.get(&path).unwrap();
            sizes
                .entry(parent.clone())
                .and_modify(|e| {
                    *e += size;
                })
                .or_insert(size);
            stack.push_back(parent);
        }
    }

    sizes
        .iter()
        .filter_map(|(item, &size)| {
            if files.contains(item) || size > 100000 {
                None
            } else {
                Some(size)
            }
        })
        .sum()
}

fn part2(input: &Input) -> usize {
    let root = Path::new("/").to_owned();

    let mut visit: Vec<PathBuf> = Vec::new();
    let mut all_children: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    let mut sizes: HashMap<PathBuf, usize> = HashMap::new();
    let mut curr_dir: PathBuf = PathBuf::new();
    for (command, outputs) in &input.command_pairs {
        match command {
            Command::Cd(dir) => {
                if *dir == "/" {
                    curr_dir = root.clone();
                } else if *dir == ".." {
                    curr_dir.pop();
                } else {
                    curr_dir.push(dir);
                }
            }
            Command::Ls => {
                visit.push(curr_dir.clone());
                let total_size = sizes.entry(curr_dir.clone()).or_insert(0);
                let children = all_children.entry(curr_dir.clone()).or_default();
                for output in outputs {
                    match output {
                        Output::Dir(dir) => {
                            let dir = curr_dir.as_path().join(dir).to_owned();
                            children.push(dir);
                        }
                        Output::File(_file, size) => {
                            *total_size += *size;
                        }
                    }
                }
            }
        }
    }

    while let Some(path) = visit.pop() {
        let children = all_children.get(&path).unwrap();
        let add_size: usize = children.iter().map(|child| sizes.get(child).unwrap()).sum();
        let total_size = sizes.entry(path).or_insert(0);
        *total_size += add_size;
    }

    let total_size = sizes.get(&root).unwrap();
    let free_space = 70_000_000 - total_size;
    let delete_at_least = 30_000_000 - free_space;

    sizes
        .values()
        .copied()
        .filter_map(|size| {
            if size < delete_at_least {
                None
            } else {
                Some(size)
            }
        })
        .min()
        .unwrap()
}

fn main() {
    let bytes = include_bytes!("input.txt");
    let input: Input = std::str::from_utf8(bytes).unwrap().into();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
