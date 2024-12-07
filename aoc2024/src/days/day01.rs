use crate::util;

pub fn run(part: u8) {
    let input = util::read_input("input/day01.txt");
    match part {
        1 => println!("{}", part1(&input)),
        2 => println!("{}", part2(&input)),
        _ => eprintln!("Invalid part: {}", part),
    }
}

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;

    // Läs in i två arrayer och sortera dom asc, räkna ut skillnaden mellan respektive index och summera

    let (list1, list2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|list| {
            let mut nums = list
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    println!("{:?}", list1);
    println!("{:?}", list2);

    //släng in en sort-operation, och en sum operation?

    5

    // Location IDs
    // Pair the smallest num in the left list with the smallest in the right
    // Then the second.. and so on
    // Measure diff between numbers in the pairs
    // What is the total diff?

    // Vad är skillnaden mellan &str, String, str?
}

fn part2(input: &str) -> i32 {
    println!("Running part 2: {}", input);
    5
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util;

    #[test]
    fn test_part1() {
        let input: String = util::read_input("input/day01-test.txt");
        assert_eq!(part1(&input), 11)
    }
}
