use std::{
    fs::File,
    path::Path,
    io::prelude::*,
    collections::HashSet,
};

const INPUT_PATH: &str = "/home/sanchace/projects/aoc/2024/day6/data/input";

type Point = (usize, usize);

enum Feedback {
    Direction(char),
    Location(Point),
    Out,
}

fn main() {
    let (mut walls, (max_row, max_col), init) = read_input(Path::new(INPUT_PATH));
    let path =  compute_path(&mut walls, (max_row, max_col), init);
    //println!("{}", path.len());
    let mut count: u32 = 0;
    for (row, col) in path {
        if !walls.contains(&(row, col)) {
            walls.insert((row, col));
            if compute_path(&mut walls, (max_row, max_col), init).is_empty() {
                count += 1;
            }
            walls.remove(&(row, col));
        }
    }
    println!("{count}");
}

fn update(walls: &mut HashSet<Point>, bounds: Point, current_location: Point, current_direction: char, visited: &mut HashSet<(Point, char)>) -> Option<Feedback> {
    if visited.contains(&(current_location, current_direction)) {return None;}
    visited.insert((current_location, current_direction));
    let new_location: Point;
    let new_direction: char;
    {
        let mut temp_new_location = current_location;
        match current_direction {
            '^' => {
                if temp_new_location.0 == 0 {
                    return Some(Feedback::Out);
                }
                temp_new_location = (temp_new_location.0 - 1, temp_new_location.1);
                new_direction = '>';
            },
            '>' => {
                if temp_new_location.1 + 1 == bounds.1 {
                    return Some(Feedback::Out);
                }
                temp_new_location = (temp_new_location.0, temp_new_location.1 + 1);
                new_direction = 'v';
            },
            'v' => {
                if temp_new_location.0 + 1 == bounds.0 {
                    return Some(Feedback::Out);
                }
                temp_new_location = (temp_new_location.0 + 1, temp_new_location.1);
                new_direction = '<';
            },
            '<' => {
                if temp_new_location.1 == 0 {
                    return Some(Feedback::Out);
                }
                temp_new_location = (temp_new_location.0, temp_new_location.1 - 1);
                new_direction = '^';
            },
            ch => panic!("unknown direction {ch}"),
        }
        new_location = temp_new_location;
    }
    if walls.contains(&new_location) {
        return Some(Feedback::Direction(new_direction));
    }
    Some(Feedback::Location(new_location))
}
fn compute_path(walls: &mut HashSet<Point>, bounds: Point, initial_position: Point) -> HashSet<Point> {
    let mut current_position = (initial_position, '^');
    let mut guard_positions: HashSet<Point> = HashSet::new();
    guard_positions.insert(current_position.0);
    let mut visited: HashSet<(Point, char)> = HashSet::new();
    while let Some(feedback) = update(walls, bounds, current_position.0, current_position.1, &mut visited) {
        match feedback {
            Feedback::Direction(ch) => {
                current_position = (current_position.0, ch);
            },
            Feedback::Location(loc) => {
                guard_positions.insert(loc);
                current_position = (loc, current_position.1);
            },
            Feedback::Out => {return guard_positions;}
        }
    }
    HashSet::<Point>::new()
}

fn read_input(path: &Path) -> (HashSet<Point>, Point, Point) {
    let mut input = String::new();
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    if let Err(why) = file.read_to_string(&mut input) {
        panic!("couldn't read {}: {}", path.display(), why);
    }
    let grid = input
        .trim()
        .lines()
        .map(|line| line.chars());
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut init: Point = (0, 0);
    let mut walls = HashSet::new();
    for line in grid {
        col = 0;
        for ch in line {
            match ch {
                '#' => {walls.insert((row, col));},
                '^' => {init = (row, col);},
                _ => (),
            }
            col += 1;
        }
        row += 1;
    }
    (walls, (row, col), init)
}
