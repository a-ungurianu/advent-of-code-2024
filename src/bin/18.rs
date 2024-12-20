use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    ops::RangeBounds,
};

use advent_of_code::Point;

advent_of_code::solution!(18);

fn in_bounds(point: &Point, n: u32) -> bool {
    return 0 <= point.0 && point.0 < n as i32 && 0 <= point.1 && point.1 < n as i32;
}
fn steps_needed(points: &[Point], n: u32) -> Option<u32> {
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut points_hm: HashMap<_, _> = points.iter().map(|p| (*p, -1)).collect();

    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(Point(0, 0));
    points_hm.insert(Point(0, 0), 0);

    while !queue.is_empty() {
        let c = queue.pop_front().unwrap();
        if c == Point(n as i32 - 1, n as i32 - 1) {
            return Some(*points_hm.get(&c).unwrap() as u32);
        }

        for dir in dirs {
            let p = &c + dir;
            if in_bounds(&p, n) {
                if !points_hm.contains_key(&p) {
                    let res = points_hm.get(&c).unwrap() + 1;
                    points_hm.insert(p, res);
                    queue.push_back(p);
                }
            }
        }
    }
    None
}
fn part_one_range(input: &str, n: u32, take: u32) -> Option<u32> {
    let points: Vec<_> = input
        .lines()
        .map(|s| {
            let [x, y] = s
                .split(",")
                .map(|p| p.parse::<i32>().unwrap())
                .collect::<Vec<_>>()[..]
            else {
                panic!("")
            };

            Point(x, y)
        })
        .collect();

    steps_needed(&points[..take as usize], n)
}

fn part_two_range(input: &str, n: u32) -> Option<Point> {
    let points: Vec<_> = input
        .lines()
        .map(|s| {
            let [x, y] = s
                .split(",")
                .map(|p| p.parse::<i32>().unwrap())
                .collect::<Vec<_>>()[..]
            else {
                panic!("")
            };

            Point(x, y)
        })
        .collect();

    let r = (0..=points.len())
        .collect::<Vec<_>>()
        .partition_point(|t| steps_needed(&points[..*t], n).is_some());

    points.get(r - 1).map(|p| *p)
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_range(input, 71, 1024)
}

pub fn part_two(input: &str) -> Option<String> {
    let r = part_two_range(input, 71).unwrap();
    Some(format!("{},{}", r.0, r.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_range(&advent_of_code::template::read_file("examples", DAY), 7, 12);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_range(&advent_of_code::template::read_file("examples", DAY), 7);
        assert_eq!(result, Some(Point(6, 1)));
    }
}
