use std::convert::identity;

use advent_of_code::Point;
use regex::*;

advent_of_code::solution!(13);

#[derive(Debug)]
struct ClawMachine {
    a_step: Point,
    b_step: Point,
    target: Point,
}

fn overshot(a: &Point, target: &Point) -> bool {
    a.0 > target.0 || a.1 > target.1
}

fn div_perfect(a: &Point, b: &Point) -> Option<i32> {
    if a.0 / b.0 == a.1 / b.1 && a.0 % b.0 == 0 && a.1 % b.1 == 0 {
        Some(a.0 / b.0)
    } else {
        None
    }
}

impl ClawMachine {
    pub fn solve(self: &Self) -> u32 {
        (0..100)
            .map(|a_count| self.try_solve(a_count))
            .filter_map(identity)
            .min()
            .unwrap_or(0)
    }

    fn try_solve(self: &Self, a_count: i32) -> Option<u32> {
        let a_hop = &self.a_step * a_count;
        if overshot(&a_hop, &self.target) {
            return None;
        } else {
            let remaining = &self.target - &a_hop;

            match div_perfect(&remaining, &self.b_step) {
                Some(b_count) => Some((a_count * 3 + b_count) as u32),
                None => None,
            }
        }
    }
}

fn get_point(line: &str) -> Option<Point> {
    let point_finder = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();

    point_finder.captures(line).and_then(|c| {
        let (_, [x, y]) = c.extract();
        Some(Point(x.parse().unwrap(), y.parse().unwrap()))
    })
}

fn parse_machine(machine_spec: &str) -> ClawMachine {
    let mut lines = machine_spec.lines();

    let a = get_point(lines.next().unwrap()).unwrap();
    let b = get_point(lines.next().unwrap()).unwrap();
    let prize = get_point(lines.next().unwrap()).unwrap();

    ClawMachine {
        a_step: a,
        b_step: b,
        target: prize,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(parse_machine)
            .map(|machine| machine.solve())
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
