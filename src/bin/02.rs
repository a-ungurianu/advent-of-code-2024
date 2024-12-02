advent_of_code::solution!(2);

#[derive(PartialEq, Clone, Copy)]
enum Order {
    Equal,
    Ascending,
    Descending,
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut prev = report[0];
    let mut order = Order::Equal;

    for current in report[1..].into_iter() {
        if order == Order::Equal && prev != *current {
            order = if prev < *current {
                Order::Ascending
            } else {
                Order::Descending
            };
        }

        if order == Order::Equal {
            return false;
        } else if order == Order::Ascending && !(prev < *current) {
            return false;
        } else if order == Order::Descending && !(prev > *current) {
            return false;
        } else if prev.abs_diff(*current) > 3 {
            return false;
        }
        prev = *current;
    }
    true
}

fn vec_copy_with_hole_at_idx(report: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut copy = report.to_vec();

    copy.remove(index);

    copy
}

fn is_report_skip_safe(report: &Vec<i32>) -> bool {
    if is_report_safe(&report) {
        return true;
    }

    for i in 0..report.len() {
        let with_hole = vec_copy_with_hole_at_idx(report, i);
        if is_report_safe(&with_hole) {
            return true;
        }
    }

    return false;
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|chunk| chunk.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });

    Some(reports.filter(|i| is_report_safe(i)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|chunk| chunk.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });

    Some(reports.filter(|i| is_report_skip_safe(i)).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_skip_increasing() {
        assert!(is_report_skip_safe(&vec![1, 3, 2, 4, 5]));
    }

    #[test]
    fn test_skip_decreasing() {
        assert!(is_report_skip_safe(&vec![8, 6, 4, 4, 1]));
    }

    #[test]
    fn test_skip_beginning() {
        assert!(is_report_skip_safe(&vec![1, 1, 2, 3, 4]));
    }

    #[test]
    fn test_skip_flip_order() {
        assert!(is_report_skip_safe(&vec![9, 6, 7, 8, 9, 10, 13, 16]))
    }
}
