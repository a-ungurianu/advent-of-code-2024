use std::ops::Index;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Point(usize, usize);

impl std::ops::Add<(i32, i32)> for &Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point(
            (self.0 as i32 + rhs.0) as usize,
            (self.1 as i32 + rhs.1) as usize,
        )
    }
}

type Grid<'a> = Vec<&'a str>;

impl Index<&Point> for Grid<'_> {
    type Output = u8;

    fn index(&self, index: &Point) -> &Self::Output {
        &self[index.0].as_bytes()[index.1]
    }
}

fn count_vertical(grid: &Vec<&str>, needle: &str) -> u32 {
    let mut count: u32 = 0;
    for row in 0..=(grid.len() - needle.len()) {
        for col in 0..grid[0].len() {
            let start = Point(row, col);
            let mut is_needle = true;
            let mut is_needle_reversed = true;
            for k in 0..needle.len() {
                let here = &start + (k as i32, 0);
                if grid[&here] != needle.as_bytes()[k] {
                    is_needle = false;
                }
                if grid[&here] != needle.as_bytes()[needle.len() - k - 1] {
                    is_needle_reversed = false;
                }
            }
            if is_needle {
                count += 1
            }
            if is_needle_reversed {
                count += 1
            }
        }
    }
    count
}

fn count_horizontal(grid: &Vec<&str>, needle: &str) -> u32 {
    let mut count: u32 = 0;
    for row in 0..grid.len() {
        for col in 0..=(grid[0].len() - needle.len()) {
            let start = Point(row, col);
            let mut is_needle = true;
            let mut is_needle_reversed = true;
            for k in 0..needle.len() {
                let here = &start + (0, k as i32);
                if grid[&here] != needle.as_bytes()[k] {
                    is_needle = false;
                }
                if grid[&here] != needle.as_bytes()[needle.len() - k - 1] {
                    is_needle_reversed = false;
                }
            }
            if is_needle {
                count += 1
            }
            if is_needle_reversed {
                count += 1
            }
        }
    }
    count
}

fn count_diagonal(grid: &Vec<&str>, needle: &str) -> u32 {
    let mut count: u32 = 0;
    for row in 0..=(grid.len() - needle.len()) {
        for col in 0..=(grid[0].len() - needle.len()) {
            let start = Point(row, col);
            let mut is_needle = true;
            let mut is_needle_reversed = true;
            for k in 0..needle.len() {
                let here = &start + (k as i32, k as i32);
                if grid[&here] != needle.as_bytes()[k] {
                    is_needle = false;
                }
                if grid[&here] != needle.as_bytes()[needle.len() - k - 1] {
                    is_needle_reversed = false;
                }
            }
            if is_needle {
                count += 1
            }
            if is_needle_reversed {
                count += 1
            }
        }
    }
    count
}

fn count_other_diagonal(grid: &Vec<&str>, needle: &str) -> u32 {
    let mut count: u32 = 0;
    for row in (needle.len() - 1)..grid.len() {
        for col in 0..=(grid[0].len() - needle.len()) {
            let start = Point(row, col);
            let mut is_needle = true;
            let mut is_needle_reversed = true;
            for k in 0..needle.len() {
                let here = &start + (-(k as i32), k as i32);
                if grid[&here] != needle.as_bytes()[k] {
                    is_needle = false;
                }
                if grid[&here] != needle.as_bytes()[needle.len() - k - 1] {
                    is_needle_reversed = false;
                }
            }
            if is_needle {
                count += 1
            }
            if is_needle_reversed {
                count += 1
            }
        }
    }
    count
}

fn count_word(grid: Vec<&str>, needle: &str) -> u32 {
    return count_vertical(&grid, needle)
        + count_diagonal(&grid, needle)
        + count_other_diagonal(&grid, needle)
        + count_horizontal(&grid, needle);
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<_> = input.lines().collect();
    Some(count_word(grid, "XMAS"))
}

pub fn part_two(input: &str) -> Option<u32> {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
