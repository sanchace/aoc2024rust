use std::fs;
use std::cmp::Ordering;

fn main() {
    let path = "data/input";
    let contents = fs::read_to_string(path).expect("Should be able to read file.");

    let mut vec_one: Vec<u32> = Vec::new();
    let mut vec_two: Vec<u32> = Vec::new();

    for line in contents.trim().lines() {
        let mut nums: Vec<u32> = line.split_whitespace().map(|x| x.parse::<u32>().expect("Should be number")).collect();
        vec_one.push(nums.pop().expect("Should be nonempty"));
        vec_two.push(nums.pop().expect("Should be nonempty"));
    }

    vec_one.sort();
    vec_two.sort();

    let mut count = 0;
    for (a, b) in vec_one.iter().zip(vec_two.iter()) {
        count += match a.cmp(b) {
            Ordering::Less => b - a,
            Ordering::Greater => a - b,
            Ordering::Equal => 0,
        };
    }
    println!("{count}");

    count = 0;
    for n in vec_one.iter() {
        for _ in vec_two.iter().filter(|x| n.eq(x)) {
            count += n;
        }
    }
    println!("{count}");
}
