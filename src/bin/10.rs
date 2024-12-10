use std::collections::{HashSet, VecDeque};

use advent_of_code::Point;

advent_of_code::solution!(10);

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn count_reached_zeros(map: &Vec<&str>, start: &Point) -> u32 {
    let mut q: VecDeque<Point> = VecDeque::from([*start]);

    let mut reached_zeros: HashSet<Point> = HashSet::new();
    while !q.is_empty() {
        let p = q.pop_front().unwrap();

        let p_height = map[&p];
        if p_height == b'0' {
            reached_zeros.insert(p);
        } else {
            for n_p in DIRS
                .map(|dir| &p + dir)
                .iter()
                .filter(|p| {
                    0 <= p.0 && p.0 < map.len() as i32 && 0 <= p.1 && p.1 < map[0].len() as i32
                })
                .filter(|next_p| map[*next_p] == p_height - 1)
            {
                q.push_back(*n_p);
            }
        }
    }
    reached_zeros.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<_> = input.lines().collect();

    let mut res = 0;

    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, _) in row.as_bytes().iter().enumerate() {
            let p = Point(row_idx as i32, col_idx as i32);
            if map[&p] == b'9' {
                res += count_reached_zeros(&map, &p);
            }
        }
    }
    Some(res)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_one_simple() {
        let result = part_one(
            "0123
1234
8765
9876",
        );
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_knots() {
        let result = part_one(
            "2290229
2221298
2222227
6543456
7652987
8762222
9872222",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
