use std::{
    collections::HashSet,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
};

const AOC_PATH: &str = "/home/sanchace/projects/aoc/2024";
const DAY_PATH: &str = "day10";
const DATA_PATH: &str = "data";
//const INPUT_PATH: &str = "sample";
const INPUT_PATH: &str = "input";

const SUMMIT_VALUE: u32 = 9;

type Point = (usize, usize);

fn main() {
    let mut path = PathBuf::from(AOC_PATH);
    path.push(DAY_PATH);
    path.push(DATA_PATH);
    path.push(INPUT_PATH);
    let input = read_input(path.as_path());
    //println!("{:?}", input);
    let input = structure_input(input);
    //println!("{}", score_all(&input));
    println!("{}", rate_all(&input));
}

fn rate_trailhead(grid: &[Vec<u32>], trailhead: Point) -> usize {
    let mut summits = 0;
    let mut queue: Vec<(Vec<Point>, Point)> = Vec::new();
    queue.push((Vec::new(), trailhead));
    while let Some((path, v)) = queue.pop() {
        if grid[v.0][v.1] == SUMMIT_VALUE {
            summits += 1;
        }
        for e in neighbors(grid, v) {
            if !path.contains(&e) {
                let mut temp = path.clone();
                temp.push(v);
                queue.push((temp, e));
            }
        }
    }
    summits
}

fn rate_all(grid: &[Vec<u32>]) -> usize {
    trailheads(grid)
        .iter()
        .map(|trailhead| rate_trailhead(grid, *trailhead))
        .sum()
}

fn neighbors(grid: &[Vec<u32>], (y, x): Point) -> Vec<Point> {
    let mut neighbors: Vec<Point> = Vec::new();
    let target = grid[y][x] + 1;
    if y > 0 && grid[y - 1][x] == target {
        neighbors.push((y - 1, x));
    }
    if y + 1 < grid.len() && grid[y+1][x] == target {
        neighbors.push((y + 1, x));
    }
    if x > 0 && grid[y][x - 1] == target {
        neighbors.push((y, x - 1));
    }
    if x + 1 < grid[y].len() && grid[y][x + 1] == target {
        neighbors.push((y, x + 1));
    }
    neighbors
}

//fn score_trailhead(grid: &[Vec<u32>], trailhead: Point) -> usize {
//    let mut summits: HashSet<Point> = HashSet::new();
//    let mut queue: Vec<(HashSet<Point>, Point)> = Vec::new();
//    queue.push((HashSet::new(), trailhead));
//    while let Some((path, v)) = queue.pop() {
//        if grid[v.0][v.1] == SUMMIT_VALUE {
//            summits.insert(v);
//        }
//        for e in neighbors(grid, v) {
//            if !path.contains(&e) {
//                let mut temp = path.clone();
//                temp.insert(v);
//                queue.push((temp, e));
//            }
//        }
//    }
//    summits.len()
//}
//
//fn score_all(grid: &[Vec<u32>]) -> usize {
//    trailheads(grid)
//        .iter()
//        .map(|trailhead| score_trailhead(grid, *trailhead))
//        .sum()
//}

fn trailheads(grid: &[Vec<u32>]) -> HashSet<Point> {
    let mut trailheads = HashSet::new();
    for (y, row)in grid.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                trailheads.insert((y, x));
            }
        }
    }
    trailheads
}

fn structure_input(s: String) -> Vec<Vec<u32>> {
    s
        .lines()
        .map(|line| line
            .chars()
            .map(|ch| ch
                .to_digit(10).expect("should be digit"))
            .collect())
        .collect()
}

fn read_input(path: &Path) -> String {
    let mut input = String::new();
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    if let Err(why) = file.read_to_string(&mut input) {
        panic!("couldn't read {}: {}", path.display(), why);
    }
    input
        .trim()
        .to_string()
}
