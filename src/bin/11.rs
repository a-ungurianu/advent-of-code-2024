use std::collections::VecDeque;

advent_of_code::solution!(11);

fn count_stones(start_stones: Vec<u64>, blinks: u32) -> u32 {
    let mut stones = VecDeque::from_iter(start_stones.iter().map(|stone| (*stone, blinks)));

    loop {
        match stones.pop_front() {
            Some((_, 0)) => break,
            Some((stone, blinks_remaining)) => {
                if stone == 0 {
                    stones.push_back((1, blinks_remaining - 1));
                } else if (stone.ilog10() + 1) % 2 == 0 {
                    // ilog10 floors the result, so we want to adjust
                    let split_10 = (10 as u64).pow((stone.ilog10() + 1) / 2);
                    stones.push_back((stone / split_10, blinks_remaining - 1));
                    stones.push_back((stone % split_10, blinks_remaining - 1));
                } else {
                    stones.push_back((stone * 2024, blinks_remaining - 1));
                }
            }
            None => break,
        }
    }
    (stones.len() + 1) as u32
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(count_stones(parse_input(input), 25))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(count_stones(parse_input(input), 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_count_stones_blink_once() {
        let result = count_stones(vec![0, 1, 10, 99, 999], 1);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_count_stones_blink_six() {
        let result = count_stones(vec![125, 17], 6);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
