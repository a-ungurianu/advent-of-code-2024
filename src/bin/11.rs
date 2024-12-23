use std::collections::HashMap;

advent_of_code::solution!(11);

fn count_stones(start_stones: Vec<u64>, blinks: u32) -> u64 {
    let mut state: HashMap<u64, u64> = HashMap::new();
    for stone in start_stones {
        *state.entry(stone).or_default() += 1;
    }
    for _ in 0..blinks {
        let mut new_state: HashMap<u64, u64> = HashMap::new();
        for (stone, amount) in state {
            if stone == 0 {
                *new_state.entry(1).or_default() += amount;
            } else if (stone.ilog10() + 1) % 2 == 0 {
                // ilog10 floors the result, so we want to adjust
                let split_10 = (10 as u64).pow((stone.ilog10() + 1) / 2);
                *new_state.entry(stone / split_10).or_default() += amount;
                *new_state.entry(stone % split_10).or_default() += amount;
            } else {
                *new_state.entry(stone * 2024).or_default() += amount;
            }
        }
        state = new_state;
    }
    state.values().sum()
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(count_stones(parse_input(input), 25))
}

pub fn part_two(input: &str) -> Option<u64> {
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
