use std::{
    path::Path,
    fs::File,
    io::prelude::*,
    collections::{HashMap, HashSet},
    cmp::Ordering,
};

const INPUT_PATH: &str = "/home/sanchace/projects/aoc/2024/day5/data/input";

fn main() {
    let mut rules_str = String::new();
    let mut updates_str = String::new();
    read_input(Path::new(INPUT_PATH), &mut rules_str, &mut updates_str);
    let rules = process_rules(rules_str);
    //println!("rules:\n{:?}\n", rules);
    let mut count = 0;
    for update in updates_str
        .lines()
        .map(|update| update
            .split(",")
            .map(|num| num
                .parse()
                .unwrap())
            .collect::<Vec<u32>>()) {
        //println!("update {}:\n{:?}\n", count, update);
        if test(&rules, update.clone()) {
            //count += update[update.len() / 2];
        } else {
            // correct order
            let new_update = reorder(&rules, update);
            count += new_update[new_update.len() / 2];
        }
    }
    println!("count: {count}");
}

fn read_input(path: &Path, rules_str: &mut String, updates_str: &mut String) {
    let mut input = String::new();
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    if let Err(why) = file.read_to_string(&mut input) {
        panic!("couldn't read {}: {}", path.display(), why);
    }
    let mut input = input
        .trim()
        .split("\n\n");
    *rules_str = input
        .next()
        .unwrap()
        .to_string();
    *updates_str = input
        .next()
        .unwrap()
        .to_string();
}

fn process_rules(rules_str: String) -> HashMap<u32, HashSet<u32>> {
    let rules_list = rules_str
    .lines()
    .map(|rule| {
        let mut rule = rule.split("|");
        (rule.next().unwrap().parse::<u32>().unwrap(),
            rule.next().unwrap().parse::<u32>().unwrap())
    });
    let mut rules_map = HashMap::<u32, HashSet<u32>>::new();
    for (rule1, rule2) in rules_list {
        rules_map
            .entry(rule2)
            .or_default()
            .insert(rule1);
    }
    rules_map
}

fn test(rules: &HashMap<u32, HashSet<u32>>, update: Vec<u32>) -> bool {
    for (idx, current) in update
        .iter()
        .enumerate() {
        let mut it = update[idx..]
            .iter();
        it.next().unwrap();
        for later in it {
            if (*rules)
                .get(current)
                .expect("Rules should be exhaustive")
                .contains(later) {
                return false;
            }
        }
    }
    true
}

fn reorder(rules: &HashMap<u32, HashSet<u32>>, update: Vec<u32>) -> Vec<u32> {
    let mut copy = update.clone();
    copy.sort_by(|a, b| {
        if a == b {return Ordering::Equal;}
        match (*rules)
            .get(a)
            .expect("Rules should be exhausive")
            .contains(b) {
            true => Ordering::Greater,
            false => Ordering::Less,
        }
    });
    copy
}
