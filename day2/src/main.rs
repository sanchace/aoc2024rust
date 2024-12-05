use std::fs;

fn main() {
    let path = "data/input";
    let contents = fs::read_to_string(path).expect("Should be able to read file");

    let mut count = 0;
    for report in contents.trim().lines() {
        if test(report.split_whitespace().map(|x| x.parse::<u32>().expect("Should be number")).collect()) {
            count += 1;
        }
    }
    println!("{count}");
}

fn test(report: Vec<u32>) -> bool {
    let mut report = report.clone();
    let mut prev = match report.pop() {
        Some(x) => x,
        None => {return false;},
    };
    let mut pop: u32 = match report.pop() {
        Some(x) => x,
        None => {return true;},
    };
    if prev == pop {
        return false;
    }
    let test = if prev < pop {
        |a: u32, b: u32| a < b && b - a <= 3
    } else {
        |a, b| a > b && a - b <= 3
    };
    loop {
        if !test(prev, pop) {return false;}
        prev = pop;
        pop = match report.pop() {
            Some(x) => x,
            None => {return true;},
        };
    }
}
