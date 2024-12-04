use std::fs;

pub fn challenge() -> u32 {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    let file = fs::read_to_string("inputs/day1.txt").expect("Unable to read file");

    for line in file.lines() {
        let mut nums = line.split_whitespace();

        let l = nums.next().unwrap().parse().unwrap();
        let r = nums.next().unwrap().parse().unwrap();

        left.push(l);
        right.push(r);
    }
    left.sort();
    right.sort();

    let mut total = 0;

    for i in 0..left.len() {
        let l = left[i];
        let r = right[i];

        if let Some(diff) = l.checked_sub(r) {
            total += diff;
        } else {
            total += r - l;
        }
    }

    println!("Total: {}", total);
    total
}
