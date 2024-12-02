use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut numbers = line
                .split_ascii_whitespace()
                .map(|chunk| chunk.parse::<u32>().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list): (Vec<_>, Vec<_>) = parse_lists(input);

    left_list.sort();
    right_list.sort();

    Some(
        left_list
            .into_iter()
            .zip(right_list.into_iter())
            .map(|(l, r)| {
                let s = l as i32 - r as i32;
                s.abs() as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list): (Vec<_>, Vec<_>) = parse_lists(input);

    let mut frequency_table: HashMap<u32, u32> = HashMap::new();

    for &item in &right_list {
        *frequency_table.entry(item).or_insert(0) += 1;
    }

    Some(left_list.into_iter().map(|val|{
        val * *frequency_table.entry(val).or_insert(0)
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
