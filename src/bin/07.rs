use std::default;

advent_of_code::solution!(7);

fn parse_row(line: &str) -> (u64, Vec<u64>) {
    let mut parts = line.split(": ");

    let target = parts.next().unwrap().parse::<u64>().unwrap();

    let components = parts
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|r| r.parse::<u64>().unwrap())
        .collect();

    (target, components)
}

fn is_doable(target: u64, components: &[u64]) -> bool {
    match components {
        [rest] => return *rest == target,
        [rest @ .., last] => {
            if *last > target {
                return false
            }
            if target % last == 0 {
                if is_doable(target / last, rest) {
                    return true
                }
            }
            is_doable(target - last, rest)
        },
        _ => false
    }
}

fn is_doable_two(target: u64, components: &[u64]) -> bool {
    match components {
        [rest] => return *rest == target,
        [rest @ .., last] => {
            if *last > target {
                return false
            }
            if target % last == 0 {
                if is_doable_two(target / last, rest) {
                    return true
                }
            }

            if target.to_string().ends_with(&last.to_string()) {
                if is_doable_two(target / (10_u64.pow(last.ilog10() + 1)), rest) {
                    return true
                }
            }
            is_doable_two(target - last, rest)
        },
        _ => false
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(parse_row)
        .filter(|(target, components)| is_doable(*target, components))
        .map(|(target, _)| target)
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(parse_row)
        .filter(|(target, components)| is_doable_two(*target, components))
        .map(|(target, _)| target)
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
