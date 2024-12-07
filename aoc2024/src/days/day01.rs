use crate::util;

pub fn run(part: u8) {
    let input = util::read_input("input/day01.txt");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => eprintln!("Invalid part: {}", part),
    }
}

fn part1(input: &str) {
    println!("Running part 1: {}", input);
}

fn part2(input: &str) {
    println!("Running part 2: {}", input);
}
