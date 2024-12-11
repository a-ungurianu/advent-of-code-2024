use advent_of_code::*;

advent_of_code::solution!(4);

fn count_vertical(grid: &Grid, needle: &str) -> u32 {
    let mut count: u32 = 0;
    for row in 0..=(grid.len() - needle.len()) {
        for col in 0..grid[0].len() {
            let start = Point(row as i32, col as i32);
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

fn count_horizontal(grid: &Grid, needle: &str) -> u32 {
    let mut count: u32 = 0;
    for row in 0..grid.len() {
        for col in 0..=(grid[0].len() - needle.len()) {
            let start = Point(row as i32, col as i32);
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

fn count_diagonal(grid: &Grid, needle: &str) -> u32 {
    let mut count: u32 = 0;
    for row in 0..=(grid.len() - needle.len()) {
        for col in 0..=(grid[0].len() - needle.len()) {
            let start = Point(row as i32, col as i32);
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

fn count_other_diagonal(grid: &Grid, needle: &str) -> u32 {
    let mut count: u32 = 0;
    for row in (needle.len() - 1)..grid.len() {
        for col in 0..=(grid[0].len() - needle.len()) {
            let start = Point(row as i32, col as i32);
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

fn count_word(grid: Grid, needle: &str) -> u32 {
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
    let grid: Vec<_> = input.lines().collect();

    let mut count: u32 = 0;

    for row in 1..(grid.len() - 1) {
        for col in 1..(grid.len() - 1) {
            let here = Point(row as i32, col as i32);
            if grid[&here] == b'A' {
                if (grid[&(&here + (-1, -1))] == b'M' && grid[&(&here + (1, 1))] == b'S')
                    || (grid[&(&here + (-1, -1))] == b'S' && grid[&(&here + (1, 1))] == b'M')
                {
                    if (grid[&(&here + (-1, 1))] == b'M' && grid[&(&here + (1, -1))] == b'S')
                        || (grid[&(&here + (-1, 1))] == b'S' && grid[&(&here + (1, -1))] == b'M')
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    Some(count)
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
