use regex::Regex;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let options: Vec<_> = lines.next().unwrap().split(", ").collect();

    let re_s = format!("^({})+$", options.join("|"));

    println!("re_s={}", re_s);

    let re = Regex::new(re_s.as_str()).unwrap();

    lines.next();

    Some(lines.filter(|s| re.is_match(s)).count() as u32)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
