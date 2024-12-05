use std::fs;

fn main() {
    let path = "data/input";
    let contents = fs::read_to_string(path).expect("Should be able to read file");

    let mut count = 0;
    for report in contents.trim().lines() {
        if test(report.split_whitespace().map(|x| x.parse::<u32>().expect("Should be number")).collect(), true) {
            count += 1;
        }
    }
    println!("{count}");
}

fn test(report: Vec<u32>, flag: bool) -> bool {
    let mut copy = report.clone();
    let mut prev = match copy.pop() {
        Some(x) => x,
        None => {return false;},
    };
    let mut pop: u32 = match copy.pop() {
        Some(x) => x,
        None => {return true;},
    };
    if prev == pop {
        return flag && test_all(report);
    }
    let test_p = if prev < pop {
        |a: u32, b: u32| a < b && b - a <= 3
    } else {
        |a, b| a > b && a - b <= 3
    };
    loop {
        if !test_p(prev, pop) {
            return flag && test_all(report);
        }
        prev = pop;
        pop = match copy.pop() {
            Some(x) => x,
            None => {return true;},
        };
    }
}

fn test_all(report: Vec<u32>) -> bool {
    for (idx, _) in report.clone().into_iter().enumerate() {
        let mut report = report.clone();
        report.remove(idx);
        if test(report, false) {return true;}
    }
    false
}
