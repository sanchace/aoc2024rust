use std::{
    collections::HashMap,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
};

const AOC_PATH: &str = "/home/sanchace/projects/aoc/2024";
const DAY_PATH: &str = "day11";
const DATA_PATH: &str = "data";
//const INPUT_PATH: &str = "sample";
const INPUT_PATH: &str = "input";

fn main() {
    let mut path = PathBuf::from(AOC_PATH);
    path.push(DAY_PATH);
    path.push(DATA_PATH);
    path.push(INPUT_PATH);
    let input = read_input(path.as_path());
    //println!("{}", input);

    let input = structure_input(input);
    //println!("{}", evolve_lst(&input, 25));
    //println!("{}", evolve_lst(&input, 75)); // this fails -- too inefficient!
    let mut count = 0;
    let mut memoize: HashMap<(u64, u8), u64> = HashMap::new();
    for num in input {
        let size = evolve_num_memo(num, 75, &mut memoize);
        count += size;
    }
    println!("{}", count);
}

fn evolve_num_memo(number: u64, iterations: u8, memo: &mut HashMap<(u64, u8), u64>) -> u64 {
    if iterations == 0 {
        return 1;
    }
    let result = match memo.get(&(number, iterations)) {
        Some(n) => *n,
        None => {
            let mut count = 0;
            for n in step_lst(&[number]) {
                count += evolve_num_memo(n, iterations - 1, memo);
            }
            count
        },
    };
    memo.insert((number, iterations), result);
    result
}

fn step_lst(lst: &[u64]) -> Vec<u64> {
    lst
        .iter()
        .flat_map(|x| {
            if *x == 0 {
                return vec![1];
            }
            let num_dig = num_digits(*x);
            if num_dig % 2 == 0 {
                let cleave = u64::pow(10, num_dig / 2);
                vec![*x / cleave, *x % cleave]
            } else {
                vec![*x * 2024]
            }
        })
        .collect()
}

fn num_digits(x: u64) -> u32 {
    if x == 0 {
        return 1;
    }
    let mut x = x;
    let mut count = 0;
    while x > 0 {
        x /= 10;
        count += 1;
    }
    count
}

//fn evolve_lst(lst: &[u64], num: usize) -> usize {
//    let mut lst = lst.to_vec();
//    for _ in 0..num {
//        lst = step_lst(&lst)
//    }
//    lst.len()
//}

fn structure_input(s: String) -> Vec<u64> {
    s
        .split_whitespace()
        .map(|ch| ch.parse().expect("should be number"))
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
