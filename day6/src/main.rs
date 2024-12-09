use std::{
    fs::File,
    path::Path,
    io::prelude::*,
    collections::HashSet,
};

const INPUT_PATH: &str = "/home/sanchace/projects/aoc/2024/day6/data/input";

enum Feedback {
    Direction(char),
    Location((usize, usize)),
}

fn main() {
    let grid = read_input(Path::new(INPUT_PATH));
    println!("{}", compute_path_size(&mut grid.clone(), find_pt(&grid, '^')));
}

fn update(grid: &mut [Vec<char>], current_coords: (char, (usize, usize))) -> Option<Feedback> {
    let direction = grid[current_coords.1.0][current_coords.1.1];
    let new_location: (usize, usize);
    let new_direction: char;
    {
        let mut temp_new_location = current_coords.1;
        match direction {
            '^' => {
                if temp_new_location.0 == 0 {return None;}
                temp_new_location = (temp_new_location.0 - 1, temp_new_location.1);
                new_direction = '>';
            },
            '>' => {
                if temp_new_location.1 + 1 == grid[temp_new_location.0].len() {return None;}
                temp_new_location = (temp_new_location.0, temp_new_location.1 + 1);
                new_direction = 'v';
            },
            'v' => {
                if temp_new_location.0 + 1 == grid.len() {return None;}
                temp_new_location = (temp_new_location.0 + 1, temp_new_location.1);
                new_direction = '<';
            },
            '<' => {
                if temp_new_location.1 == 0 {return None;}
                temp_new_location = (temp_new_location.0, temp_new_location.1 - 1);
                new_direction = '^';
            },
            ch => panic!("unknown direction {ch}"),
        }
        new_location = temp_new_location;
    }
    if grid[new_location.0][new_location.1] == '#' {
        grid[current_coords.1.0][current_coords.1.1] = new_direction;
        return Some(Feedback::Direction(new_direction));
    }
    grid[current_coords.1.0][current_coords.1.1] = 'X';
    grid[new_location.0][new_location.1] = direction;
    Some(Feedback::Location(new_location))
}

fn compute_path_size(grid: &mut [Vec<char>], initial_position: (usize, usize)) -> usize {
    let mut current_position = ('^', initial_position);
    let mut guard_positions: HashSet<(usize, usize)> = HashSet::new();
    guard_positions.insert(current_position.1);
    while let Some(feedback) = update(grid, current_position) {
        match feedback {
            Feedback::Direction(ch) => {
                current_position = (ch, current_position.1);
            },
            Feedback::Location(loc) => {
                guard_positions.insert(loc);
                current_position = (current_position.0, loc);
            },
        }
    }
    guard_positions.len()
}

fn find_pt(grid: &[Vec<char>], direction: char) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == direction {
                return (y, x);
            }
        }
    }
    panic!("Did not find \'{direction}\' in grid!");
}

fn read_input(path: &Path) -> Vec<Vec<char>> {
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
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
