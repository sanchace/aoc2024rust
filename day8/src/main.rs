use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::prelude::*,
    path::Path,
};

const INPUT_PATH: &str = "/home/sanchace/projects/aoc/2024/day8/data/input";

type Point = (usize, usize);

fn main() {
    let (bounds, antennae) = read_input(Path::new(INPUT_PATH));
    let mut locations: HashSet<Point> = HashSet::new();
    for (ch, points) in antennae.into_iter() {
        if ch != '.' {
            // figure out how to do this properly
            let vec: Vec<Point> = points.into_iter().collect();
            let n = vec.len();
            for (i, j) in (0..n).flat_map(|i| (i + 1..n).map(move |j| (i, j))) {
                let p = *vec.get(i).expect("should be in bounds");
                let q = *vec.get(j).expect("should be in bounds");
                for antinode in resonant_antinodes(p, q, bounds) {
                    locations.insert(antinode);
                }
            }
        }
    }
    println!("{}", locations.len());
}

fn resonant_antinodes(p: Point, q: Point, bounds: Point) -> Vec<Point> {
    let mut antis = Vec::new();
    let mut i: usize = 0;
    loop {
        if (i + 1) * p.0 >= i * q.0
            && (i + 1) * p.1 >= i * q.1
            && (i + 1) * p.0 < i * q.0 + bounds.0
            && (i + 1) * p.1 < i * q.1 + bounds.1
        {
            antis.push(((i + 1) * p.0 - i * q.0, (i + 1) * p.1 - i * q.1));
        } else {
            break;
        }
        i += 1;
    }
    i = 0;
    loop {
        if (i + 1) * q.0 >= i * p.0
            && (i + 1) * q.1 >= i * p.1
            && (i + 1) * q.0 < i * p.0 + bounds.0
            && (i + 1) * q.1 < i * p.1 + bounds.1
        {
            antis.push(((i + 1) * q.0 - i * p.0, (i + 1) * q.1 - i * p.1));
        } else {
            break;
        }
        i += 1;
    }
    antis
}

fn read_input(path: &Path) -> (Point, HashMap<char, HashSet<Point>>) {
    let mut input = String::new();
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    if let Err(why) = file.read_to_string(&mut input) {
        panic!("couldn't read {}: {}", path.display(), why);
    }
    let mut row_idx = 0;
    let mut col_idx = 0;
    let mut antennae: HashMap<char, HashSet<Point>> = HashMap::new();
    for row in input.trim().lines().map(|line| line.chars()) {
        col_idx = 0;
        for ch in row {
            antennae.entry(ch).or_default().insert((row_idx, col_idx));
            col_idx += 1;
        }
        row_idx += 1;
    }
    ((row_idx, col_idx), antennae)
}
