advent_of_code::solution!(3);

use regex::*;

pub fn part_one(input: &str) -> Option<u32> {
    let mul_finder = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let res = mul_finder
        .captures_iter(input)
        .map(|caps| {
            let (_, [a_s, b_s]): (&str, [&str; 2]) = caps.extract();
            a_s.parse::<u32>().unwrap() * b_s.parse::<u32>().unwrap()
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mul_finder = Regex::new(r"don\'t\(\)()()|do\(\)()()|mul\((\d+),(\d+)\)").unwrap();

    let mut enabled = true;
    let res = mul_finder
        .captures_iter(input)
        .map(|caps| {
            match caps.extract() {
                ("do()", _) => {
                    enabled = true;
                    0
                }
                ("don't()", _) => {
                    enabled = false;
                    0
                }
                (_, [a_s, b_s]) => {
                    if enabled {
                        a_s.parse::<u32>().unwrap() * b_s.parse::<u32>().unwrap()
                    }
                    else {
                        0
                    }
                }
            }
        })
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
