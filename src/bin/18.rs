use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use advent_of_code::Point;

advent_of_code::solution!(18);

fn in_bounds(point: &Point, n: u32) -> bool {
    return 0 <= point.0 && point.0 < n as i32 && 0 <= point.1 && point.1 < n as i32;
}

fn part_one_range(input: &str, n: u32, take: u32) -> u32 {
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut points: HashMap<_, _> = input
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
        .map(|p| (p, -1))
        .take(take as usize)
        .collect();

    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(Point(0, 0));
    points.insert(Point(0, 0), 0);

    while !queue.is_empty() {
        let c = queue.pop_front().unwrap();
        if c == Point(n as i32 - 1, n as i32 - 1) {
            return *points.get(&c).unwrap() as u32;
        }

        for dir in dirs {
            let p = &c + dir;
            if in_bounds(&p, n) {
                if !points.contains_key(&p) {
                    let res = points.get(&c).unwrap() + 1;
                    points.insert(p, res);
                    queue.push_back(p);
                }
            }
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(part_one_range(input, 71, 1024))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_range(&advent_of_code::template::read_file("examples", DAY), 7, 12);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
