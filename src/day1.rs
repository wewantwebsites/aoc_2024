use std::{collections::HashMap, fs};

pub struct Towers {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Towers {
    pub fn from_file(path: &str) -> Self {
        let mut left: Vec<u32> = vec![];
        let mut right: Vec<u32> = vec![];
        let file = fs::read_to_string(path).expect("Unable to read file");

        for line in file.lines() {
            let mut nums = line.split_whitespace();

            let l = nums.next().unwrap().parse().unwrap();
            let r = nums.next().unwrap().parse().unwrap();

            left.push(l);
            right.push(r);
        }
        left.sort();
        right.sort();

        Self { left, right }
    }
}

pub fn part1(towers: &Towers) -> u32 {
    let mut total = 0;

    for i in 0..towers.left.len() {
        let l = towers.left[i];
        let r = towers.right[i];

        if let Some(diff) = l.checked_sub(r) {
            total += diff;
        } else {
            total += r - l;
        }
    }

    println!("Part 1: {}", total);
    total
}

pub fn part2(towers: &Towers) -> u32 {
    let mut counter: HashMap<u32, u32> = HashMap::new();
    let mut total = 0;

    for i in 0..towers.left.len() {
        let l = towers.left[i];

        for j in 0..towers.right.len() {
            let r = towers.right[j];
            if l == r {
                if let Some(value) = counter.insert(l, 1) {
                    counter.insert(l, value + 1);
                }
            }
        }

        if let Some(value) = counter.get(&l) {
            total += value * l;
        }
    }

    println!("Part 2: {}", total);
    total
}
