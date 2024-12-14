use std::collections::HashMap;

use advent_of_code::Point;
use regex::Regex;

advent_of_code::solution!(14);

struct Robot {
    start: Point,
    velocity: Point,
}

fn simulate_robot(robot: &Robot, no_steps: i32, grid_width: u32, grid_height: u32) -> Point {
    let res = &robot.start + &robot.velocity * no_steps;

    Point(
        res.0.rem_euclid(grid_width as i32),
        res.1.rem_euclid(grid_height as i32),
    )
}

fn print_positions(points: &Vec<Point>, grid_width: u32, grid_height: u32) -> bool {
    let mut map: HashMap<&Point, u32> = HashMap::new();
    for point in points {
        *map.entry(&point).or_default() += 1;
    }

    let mut max_c = 0;

    for y in 0..grid_height {
        let mut c = 0;
        for x in 0..grid_width {
            match map.get(&Point(x as i32, y as i32)) {
                Some(_) => c += 1,
                None => {
                    max_c = max_c.max(c);
                    c = 0;
                }
            }
        }
    }

    if max_c > 20 {
        for y in 0..grid_height {
            for x in 0..grid_width {
                match map.get(&Point(x as i32, y as i32)) {
                    Some(_) => print!("#"),
                    None => print!(" "),
                }
            }
            println!();
        }
        true
    } else {
        false
    }
}

fn simulate_robots(
    robots: &Vec<Robot>,
    steps: i32,
    grid_width: u32,
    grid_height: u32,
) -> Vec<Point> {
    robots
        .iter()
        .map(|robot| simulate_robot(&robot, steps, grid_width, grid_height))
        .collect()
}

fn part_one_with_grid_size(input: &str, width: u32, height: u32) -> u32 {
    let input_regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let robots = input_regex.captures_iter(input).map(|captures| {
        let (_, [px, py, vx, vy]) = captures.extract();

        Robot {
            start: Point(px.parse().unwrap(), py.parse().unwrap()),
            velocity: Point(vx.parse().unwrap(), vy.parse().unwrap()),
        }
    });

    let positions = robots.map(|robot| simulate_robot(&robot, 100, width, height));

    let (q1, q2, q3, q4) = positions.fold((0, 0, 0, 0), |acc, pos| {
        let x_mid = width / 2;
        let y_mid = height / 2;

        if pos.0 < x_mid as i32 && pos.1 < y_mid as i32 {
            (acc.0 + 1 as u32, acc.1, acc.2, acc.3)
        } else if pos.0 > x_mid as i32 && pos.1 < y_mid as i32 {
            (acc.0, acc.1 + 1, acc.2, acc.3)
        } else if pos.0 < x_mid as i32 && pos.1 > y_mid as i32 {
            (acc.0, acc.1, acc.2 + 1, acc.3)
        } else if pos.0 > x_mid as i32 && pos.1 > y_mid as i32 {
            (acc.0, acc.1, acc.2, acc.3 + 1)
        } else {
            acc
        }
    });
    q1 * q2 * q3 * q4
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(part_one_with_grid_size(input, 101, 103))
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let robots = input_regex
        .captures_iter(input)
        .map(|captures| {
            let (_, [px, py, vx, vy]) = captures.extract();

            Robot {
                start: Point(px.parse().unwrap(), py.parse().unwrap()),
                velocity: Point(vx.parse().unwrap(), vy.parse().unwrap()),
            }
        })
        .collect();

    for steps in 0..10000 {
        if print_positions(&simulate_robots(&robots, steps, 101, 103), 101, 103) {
            println!("Step {}", steps);
            return Some(steps as u32)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one_with_grid_size(&advent_of_code::template::read_file("examples", DAY), 11, 7);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
