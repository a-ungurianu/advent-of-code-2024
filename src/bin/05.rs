use std::collections::HashMap;

advent_of_code::solution!(5);

type Rule = (u32, u32);

type Report = Vec<u32>;

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Report>) {
    let mut line_iter = input.lines();
    let rules_s = line_iter.by_ref().take_while(|line| line.len() > 0);

    let rules: Vec<Rule> = rules_s
        .map(|line| {
            let mut parts = line.split('|').map(|s| s.parse::<u32>().unwrap());

            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();
    let reports: Vec<Report> = line_iter
        .map(|line| {
            let parts = line.split(',');
            parts.map(|part| part.parse::<u32>().unwrap()).collect()
        })
        .collect();
    (rules, reports)
}

fn matches_rules(report: &Report, rules: &Vec<Rule>) -> bool {
    let index_map: HashMap<u32, usize> = report
        .iter()
        .enumerate()
        .map(|(idx, v)| (*v, idx))
        .collect();

    rules.iter().all(
        |(left, right)| match (index_map.get(left), index_map.get(right)) {
            (Some(left_idx), Some(right_idx)) => left_idx < right_idx,
            _ => true,
        },
    )
}

fn find_middle(report: &Report) -> u32 {
    report[report.len() / 2]
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, reports) = parse_input(input);
    Some(
        reports
            .iter()
            .filter(|report| matches_rules(report, &rules))
            .map(find_middle)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, reports) = parse_input(input);
    Some(
        reports
            .iter()
            .filter(|report| !matches_rules(report, &rules))
            .map(|report| fix_report(report, &rules))
            .map(|fixed_report| find_middle(&fixed_report))
            .sum(),
    )
}

fn fix_report_step(report: &mut Report, rules: &Vec<Rule>) -> bool {
    let mut index_map: HashMap<u32, usize> = report
        .iter()
        .enumerate()
        .map(|(idx, v)| (*v, idx))
        .collect();

    let mut swapped = false;
    for (left, right) in rules {
        match (index_map.get(left), index_map.get(right)) {
            (Some(&left_idx), Some(&right_idx)) => {
                if left_idx > right_idx {
                    report.swap(left_idx, right_idx);
                    index_map.insert(report[left_idx], left_idx);
                    index_map.insert(report[right_idx], right_idx);
                    swapped = true;
                }
            }
            _ => (),
        }
    }
    return swapped;
}

fn fix_report(report: &Report, rules: &Vec<Rule>) -> Report {
    let mut fixed_report = report.clone();
    while fix_report_step(&mut fixed_report, rules) {}
    fixed_report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
