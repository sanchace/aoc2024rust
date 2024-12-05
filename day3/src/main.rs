use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("data/input").expect("Should be able to read file");

    let re = Regex::new(r"mul\((?<l>\d+),(?<r>\d+)\)|do\(\)|don\'t\(\)").unwrap();
    
    //let mut count = 0;
    for mul in re.captures_iter(&contents) {
        //let left = &mul["l"].parse::<u64>().expect("Should be number");
        //let right = &mul["r"].parse::<u64>().expect("Should be number");
        //count += left * right;
        let s = &mul[0];
        println!("{s}");
    }
    //println!("{count}");
}
