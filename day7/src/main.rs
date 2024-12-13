use std::{fs::File, io::prelude::*, path::Path};

const INPUT_PATH: &str = "/home/sanchace/projects/aoc/2024/day7/data/input";

fn main() {
    let mut count: u64 = 0;
    for equation in &read_input(Path::new(INPUT_PATH)) {
        if solvable(equation, 3) {
            count += equation.0;
        }
    }
    println!("{}", count);
}

fn int_len(n: u64, base: u8) -> u8 {
    let base: u64 = base.into();
    if n == 0 {
        return 1;
    }
    let mut n = n;
    let mut l = 0;
    while n != 0 {
        n /= base;
        l += 1;
    }
    l
}

fn int_concat_dec(a: u64, b: u64) -> u64 {
    const BASE: u8 = 10;
    let base: u64 = BASE.into();
    if b == 0 {
        return a * base;
    }
    let mut a = a;
    let mut b = b;
    let mut l = int_len(b, BASE);
    while l != 0 {
        a *= base;
        a += b / u64::pow(base, (l - 1).into());
        b %= u64::pow(base, (l - 1).into());
        l -= 1;
    }
    a
}

fn get_operator(code: u64, idx: u32, switch: u8) -> impl Fn(u64, u64) -> u64 {
    let switch: u64 = switch.into();
    match code / u64::pow(switch, idx) % switch {
        0 => move |a, b| a + b,
        1 => move |a, b| a * b,
        2 => move |a, b| int_concat_dec(a,b),
        _ => panic!("operation not implemented"),
    }
}

fn compute_expression(code: u64, lst: &[u64], switch: u8) -> u64 {
    let mut lst = lst.iter();
    let mut total: u64 = *lst.next().expect("lst should be nonempty");
    for (idx, &num) in lst.enumerate() {
        total = get_operator(code, idx.try_into().unwrap(), switch)(total, num);
    }
    total
}

fn solvable(equation: &(u64, Vec<u64>), switch: u8) -> bool {
    if equation.1.is_empty() {
        return false;
    }
    for code in 0..u64::pow(switch.into(), (equation.1.len() - 1).try_into().unwrap()) {
        if equation.0 == compute_expression(code, &equation.1, switch) {
            //println!("{code:b}, {:?}", equation);
            return true;
        }
    }
    false
}

fn read_input(path: &Path) -> Vec<(u64, Vec<u64>)> {
    let mut input = String::new();
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    if let Err(why) = file.read_to_string(&mut input) {
        panic!("couldn't read {}: {}", path.display(), why);
    }
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let test_value: u64 = parts
                .next()
                .expect("should be test value")
                .parse()
                .expect("test value should be int");
            (
                test_value,
                parts
                    .next()
                    .expect("should be list of nums")
                    .split_whitespace()
                    .map(|num| num.parse().expect("num should be int"))
                    .collect(),
            )
        })
        .collect()
}
