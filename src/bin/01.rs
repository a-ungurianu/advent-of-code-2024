advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut numbers = line
                .split_ascii_whitespace()
                .map(|chunk| chunk.parse::<u32>().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .unzip();
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
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
