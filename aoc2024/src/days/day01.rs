use crate::util;

fn part1(input: &str) -> i32 {
    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = create_vectors_from_col1_col2(input);

    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part2(input: &str) -> i32 {
    let (list1, list2): (Vec<i32>, Vec<i32>) = create_vectors_from_col1_col2(input);

    list1
        .iter()
        .map(|num| num * list2.iter().filter(|&num2| num == num2).count() as i32)
        .sum()
}

fn create_vectors_from_col1_col2(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|list| {
            let mut nums = list
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip()
}

pub fn run(part: u8) {
    let input = util::read_input("input/day01.txt");
    match part {
        1 => println!("{}", part1(&input)),
        2 => println!("{}", part2(&input)),
        _ => eprintln!("Invalid part: {}", part),
    }
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

    #[test]
    fn test_part2() {
        let input: String = util::read_input("input/day01-test.txt");
        assert_eq!(part2(&input), 31)
    }
}
