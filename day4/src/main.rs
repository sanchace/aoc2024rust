use std::fs;

const PATH: &str = "data/input";
const FLAG: bool = true;

fn main() {
    let content = fs::read_to_string(PATH).expect("Should be able to read file");

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in content.trim().lines() {
        let mut temp = Vec::new();
        for ch in line.chars() {
            temp.push(ch);
        }
        grid.push(temp);
    }

    let mut count = 0;
    for (row, line) in grid.clone().into_iter().enumerate() {
        for (col, _ch) in line.into_iter().enumerate() {
            count += dispatch(&grid, row, col, FLAG);
        }
    }
    println!("{count}");
}

fn dispatch(grid: &[Vec<char>], row: usize, col: usize, flag: bool) -> u32 {
    if flag {
        return x_mas(grid, row, col) as u32;
    }
    xmas(grid, row, col)
}


fn xmas(grid: &[Vec<char>], row: usize, col: usize) -> u32 {
    let dirs: [(i8, i8); 8] = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
    let mut count = 0;
    for (dx, dy) in dirs {
        if count_dir(grid, row, col, dx, dy) {
            count += 1;
        }
    }
    count
}

fn count_dir(grid: &[Vec<char>], row: usize, col: usize, dx: i8, dy: i8) -> bool {
    const WORD: &str = "XMAS";

    let max_row = grid.len() as i32;
    let max_col = grid[0].len() as i32;
    for (idx, ch) in WORD.chars().enumerate() {
        let y_new: i32 = row as i32 + dy as i32 * idx as i32;
        if y_new < 0 || max_row <= y_new {return false;}
        let x_new: i32 = col as i32 + dx as i32 * idx as i32;
        if x_new < 0 || max_col <= x_new {return false;}
        let grid_ch = grid[y_new as usize][x_new as usize];
        if grid_ch != ch {return false;}
    }
    true
}

fn x_mas(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    const X: [char; 2] = ['M', 'S'];

    let max_row = grid.len();
    if row == 0 || row == max_row - 1 {return false;}
    let max_col = grid[0].len();
    if col == 0 || col == max_col - 1 {return false;}
    if grid[row][col] != 'A' {return false;}
    for switch in [true, false] {
        let y1 = match switch {
            true => row - 1,
            false => row + 1,
        };
        let x1 = col - 1;
        let y2 = match switch {
            true => row + 1,
            false => row - 1,
        };
        let x2 = col + 1;
        let grid_ch1 = grid[y1][x1];
        let grid_ch2 = grid[y2][x2];
        let test = grid_ch1 == X[0] && grid_ch2 == X[1] || grid_ch1 == X[1] && grid_ch2 == X[0];
        if !test {return false;}
    }
    true
}
