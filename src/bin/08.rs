use core::str;
use std::collections::{HashMap, HashSet};

use advent_of_code::Point;

advent_of_code::solution!(8);

fn in_bounds(point: &Point, height: i32, width: i32) -> bool {
    0 <= point.0 && point.0 < height && 0 <= point.1 && point.1 < width
}

pub fn part_one(input: &str) -> Option<u32> {
    let height = input.lines().count() as i32;
    let width = input
        .lines()
        .next()
        .and_then(|line| Some(line.bytes().count()))
        .unwrap_or(0) as i32;

    let antennas = input.lines().enumerate().flat_map(|(row, line)| {
        line.bytes()
            .enumerate()
            .filter_map(move |(col, c)| match c {
                b'.' => None,
                c => Some((c, Point(row as i32, col as i32))),
            })
    });

    let mut grouped_antennas: HashMap<u8, Vec<Point>> = HashMap::new();

    for (freq, location) in antennas {
        grouped_antennas.entry(freq).or_default().push(location);
    }

    let mut antinotes: HashSet<Point> = HashSet::new();

    for (_, antenna_group) in grouped_antennas.values().enumerate() {
        for (i, antenna_a) in antenna_group.iter().enumerate() {
            for antenna_b in antenna_group[(i + 1)..].iter() {
                let antinode_a = &(antenna_a * 2) - antenna_b;
                let antinode_b = &(antenna_b * 2) - antenna_a;

                if in_bounds(&antinode_a, height, width) {
                    antinotes.insert(antinode_a);
                }

                if in_bounds(&antinode_b, height, width) {
                    antinotes.insert(antinode_b);
                }
            }
        }
    }
    Some(antinotes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let height = input.lines().count() as i32;
    let width = input
        .lines()
        .next()
        .and_then(|line| Some(line.bytes().count()))
        .unwrap_or(0) as i32;

    let antennas = input.lines().enumerate().flat_map(|(row, line)| {
        line.bytes()
            .enumerate()
            .filter_map(move |(col, c)| match c {
                b'.' => None,
                c => Some((c, Point(row as i32, col as i32))),
            })
    });

    let mut grouped_antennas: HashMap<u8, Vec<Point>> = HashMap::new();

    for (freq, location) in antennas {
        grouped_antennas.entry(freq).or_default().push(location);
    }

    let mut antinotes: HashSet<Point> = HashSet::new();

    for (_, antenna_group) in grouped_antennas.values().enumerate() {
        for (i, antenna_a) in antenna_group.iter().enumerate() {
            for antenna_b in antenna_group[(i + 1)..].iter() {
                let diff_a = antenna_b - antenna_a;
                let mut antinode_a = antenna_a.clone();

                while in_bounds(&antinode_a, width, height) {
                    antinotes.insert(antinode_a);
                    antinode_a = &antinode_a - &diff_a;
                }

                let diff_b = antenna_a - antenna_b;
                let mut antinode_b = antenna_b.clone();

                while in_bounds(&antinode_b, width, height) {
                    antinotes.insert(antinode_b);
                    antinode_b = &antinode_b - &diff_b;
                }
            }
        }
    }
    Some(antinotes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_one_on_the_border() {
        let result = part_one(
            "..X..
...X.
.....
.....",
        );
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(
            "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
