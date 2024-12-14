use std::{
    collections::VecDeque,
    fs::File,
    io::prelude::*,
    iter::repeat,
    path::{Path, PathBuf},
};

const DATA_PATH: &str = "/home/sanchace/projects/aoc/2024/day9/data";
const INPUT_PATH: &str = "input";
//const SAMPLE_PATH: &str = "sample";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Digit(usize),
    Space(),
}

type Structure = (VecDeque<(usize, u8)>, (u8, u8));

fn main() {
    let mut path = PathBuf::from(DATA_PATH);
    path.push(INPUT_PATH);
    let nums: Vec<u8> = read_input(path.as_path());
    //let memory: Vec<Symbol> = symbolize(&nums);
    //let reduced_memory = reduce_mem(&memory);
    let reduced_memory = reduce_nums(&nums);
    println!("{}", compute_checksum(&reduced_memory));
}

fn reduce_nums(nums: &[u8]) -> Vec<Symbol> {
    let mut lst = structure(nums);
    for idx in (0..lst.len()).rev() {
        let (left, right) = lst.split_at_mut(idx);
        let (ref mut files, (ref mut before, _)) = right.first_mut().unwrap();
        let (id, size) = files.pop_front().expect("Should be nonempty");
        let mut moved = false;
        for (ref mut files, (_, ref mut space)) in left.iter_mut() {
            if size <= *space {
                moved = true;
                *before += size;
                *space -= size;
                files.push_back((id, size));
                break;
            }
        }
        if !moved {
            files.push_front((id, size));
        }
    }
    destructure(&lst)
}

fn destructure(nums: &[Structure]) -> Vec<Symbol> {
    let temp = nums
        .iter()
        .flat_map(|(files, (before, after))| {
            repeat(Symbol::Space()).take(*before as usize).chain(
                files
                    .iter()
                    .flat_map(|(id, size)| repeat(Symbol::Digit(*id)).take(*size as usize))
                    .chain(repeat(Symbol::Space()).take(*after as usize)),
            )
        })
        .collect();
    temp
}

fn structure(nums: &[u8]) -> Vec<Structure> {
    let mut it = nums.iter();
    let mut lst: Vec<Structure> = Vec::new();
    let mut idx = 0;
    loop {
        let file_size = match it.next() {
            Some(&num) => num,
            None => {
                return lst;
            }
        };
        let space = match it.next() {
            Some(&num) => num,
            None => 0,
        };
        lst.push((VecDeque::from([(idx, file_size)]), (0, space)));
        idx += 1;
    }
}

fn compute_checksum(memory: &[Symbol]) -> u64 {
    memory
        .iter()
        .enumerate()
        .filter(|(_, &s)| match s {
            Symbol::Digit(_) => true,
            Symbol::Space() => false,
        })
        .map(|(idx, &s)| match s {
            Symbol::Digit(n) => (idx, n),
            Symbol::Space() => panic!("should be no spaces left"),
        })
        .map(|(idx, num)| {
            let temp: u64 = (idx * num).try_into().expect("archetecture too big");
            temp
        })
        .sum()
}

//fn reduce_mem(memory: &[Symbol]) -> Vec<Symbol> {
//    let mut reduced_memory = memory.to_owned().clone();
//    move_files(
//        find_next_space(&reduced_memory, 0).expect("no spaces in memory"),
//        find_prev_file(&reduced_memory, reduced_memory.len()).expect("no files in memory"),
//        &mut reduced_memory,
//    );
//    reduced_memory.to_vec()
//}
//
//fn symbolize(nums: &[u8]) -> Vec<Symbol> {
//    nums.iter()
//        .enumerate()
//        .flat_map(|(idx, &num)| {
//            if idx % 2 == 0 {
//                repeat(Symbol::Digit(idx / 2)).take(num.into())
//            } else {
//                repeat(Symbol::Space()).take(num.into())
//            }
//        })
//        .collect()
//}
//
//fn move_files(first_space: usize, last_file: usize, memory: &mut [Symbol]) {
//    let mut s = first_space;
//    let mut f = last_file;
//    while s < f {
//        memory[s] = memory[f];
//        memory[f] = Symbol::Space();
//        s = find_next_space(memory, s).unwrap();
//        f = find_prev_file(memory, f).unwrap();
//    }
//}
//
//fn find_prev_file(mem: &[Symbol], next: usize) -> Option<usize> {
//    for (idx, s) in mem[..next].iter().enumerate().rev() {
//        if *s != Symbol::Space() {
//            return Some(idx);
//        }
//    }
//    None
//}
//
//fn find_next_space(mem: &[Symbol], prev: usize) -> Option<usize> {
//    for (idx, s) in mem[prev + 1..].iter().enumerate() {
//        if *s == Symbol::Space() {
//            return Some(idx + prev + 1);
//        }
//    }
//    None
//}

fn read_input(path: &Path) -> Vec<u8> {
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
        .chars()
        .map(|ch| ch.to_digit(10).unwrap().try_into().unwrap())
        .collect()
}
