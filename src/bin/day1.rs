use aoc_2024::day1::{part1, part2, Towers};

fn main() {
    let towers = Towers::from_file("inputs/day1.txt");
    part1(&towers);
    part2(&towers);
}
