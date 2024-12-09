use std::{collections::HashSet, default};

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl std::ops::Add<(i32, i32)> for &Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn rotate_90(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn step(point: &Point, dir: &Direction) -> Point {
    match dir {
        Direction::Up => point + (-1, 0),
        Direction::Right => point + (0, 1),
        Direction::Down => point + (1, 0),
        Direction::Left => point + (0, -1),
    }
}

#[derive(Debug)]
enum GetResult {
    Obstacle,
    Free,
    OutOfBounds,
}

fn get(map: &Vec<&str>, point: Point) -> GetResult {
    if point.0 < 0 || point.1 < 0 {
        return GetResult::OutOfBounds;
    }

    match map.get(point.0 as usize) {
        None => GetResult::OutOfBounds,
        Some(row) => match row.as_bytes().get(point.1 as usize) {
            None => GetResult::OutOfBounds,
            Some(c) => match c {
                b'#' => GetResult::Obstacle,
                _ => GetResult::Free,
            },
        },
    }
}

fn step_guard(
    obstacles: &Vec<&str>,
    guard_pos: &Point,
    guard_dir: &Direction,
) -> Option<(Direction, Point)> {
    let next = step(guard_pos, guard_dir);

    match (get(obstacles, next)) {
        GetResult::OutOfBounds => None,
        GetResult::Free => Some((*guard_dir, next)),
        GetResult::Obstacle => Some((rotate_90(guard_dir), *guard_pos)),
    }
}

fn count_guard_steps(obstacles: &Vec<&str>, start_guard_pos: Point) -> u32 {
    let mut guard_pos = start_guard_pos;
    let mut guard_dir = Direction::Up;
    let mut visited_points = HashSet::new();

    loop {
        match step_guard(obstacles, &guard_pos, &guard_dir) {
            None => break,
            Some(res) => {
                (guard_dir, guard_pos) = res;
                visited_points.insert(guard_pos);
            }
        }
    }

    visited_points.len() as u32
}

fn parse_input(input: &str) -> (Point, Vec<&str>) {
    let mut guard: Option<Point> = None;
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, c) in row.as_bytes().iter().enumerate() {
            match c {
                b'^' => {
                    guard = Some(Point(row_idx as i32, col_idx as i32));
                    break;
                }
                _ => {}
            }
        }
    }
    (guard.unwrap(), input.lines().collect())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (guard, map) = parse_input(input);
    Some(count_guard_steps(&map, guard))
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
