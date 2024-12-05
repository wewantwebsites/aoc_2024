use std::fs;

enum Trend {
    Increasing,
    Decreasing,
}

pub fn part1() {
    println!("Part 1: ");

    let file = fs::read_to_string("inputs/day2.txt").expect("There was an issue reading the file.");
    let mut total = 0;

    'lines: for line in file.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|s| {
                let n: u32 = s.parse().unwrap();
                n
            })
            .collect();

        let trend;
        if numbers[0] > numbers[numbers.len() - 1] {
            trend = Trend::Decreasing;
        } else {
            trend = Trend::Increasing;
        }

        for i in 1..numbers.len() {
            let prev = numbers[i - 1];
            let curr = numbers[i];

            match trend {
                Trend::Increasing => {
                    if prev > curr {
                        continue 'lines;
                    }
                }
                Trend::Decreasing => {
                    if prev < curr {
                        continue 'lines;
                    }
                }
            }

            let diff = curr.abs_diff(prev);
            if diff < 1 || diff > 3 {
                continue 'lines;
            }
        }

        total = &total + 1;
    }

    println!("{}", total);
}

pub fn part2() {
    println!("Part 2: ");
}
