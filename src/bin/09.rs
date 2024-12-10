use std::collections::VecDeque;

use advent_of_code::template::aoc_cli::check;

advent_of_code::solution!(9);

type Partition = (u8, u32, bool);

fn parse_partitions<T: FromIterator<Partition>>(input: &str) -> T {
    input
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(idx, c)| (c - b'0', (idx as u32) / 2, idx % 2 == 0))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut partitions: VecDeque<_> = parse_partitions(input);

    let mut data_idx: u32 = 0;
    let mut checksum: u64 = 0;

    while !partitions.is_empty() {
        let (mut len, block_idx, is_data) = partitions.pop_front().unwrap();

        if len == 0 {
        } else if is_data {
            checksum += (data_idx * block_idx) as u64;
            data_idx += 1;
            len -= 1;
            partitions.push_front((len, block_idx, is_data));
        } else {
            let (mut len_back, block_idx_back, is_data_back) = partitions.pop_back().unwrap();

            if len_back == 0 || !is_data_back {
                partitions.push_front((len, block_idx, is_data));
            } else {
                len -= 1;
                len_back -= 1;

                checksum += (data_idx * block_idx_back) as u64;
                data_idx += 1;
                partitions.push_front((len, block_idx, is_data));
                partitions.push_back((len_back, block_idx_back, is_data_back));
            }
        }
    }
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut partitions: VecDeque<_> = parse_partitions(input);

    let mut data_idx: u32 = 0;
    let mut checksum = 0;
    while !partitions.is_empty() {
        let (len, block_idx, is_data) = partitions.pop_front().unwrap();
        if is_data {
            checksum += (0..len)
                .map(|i| ((data_idx + i as u32) * block_idx) as u64)
                .sum::<u64>();
            data_idx += len as u32;
        } else {
            let candidate = partitions
                .iter()
                .enumerate()
                .rev()
                .filter(|(_, partition)| partition.2 && (partition.0 <= len))
                .next();
            match candidate {
                Some((idx, partition)) => {
                    let rest = len - partition.0;

                    checksum += (0..partition.0)
                        .map(|i| ((data_idx + i as u32) * partition.1) as u64)
                        .sum::<u64>();
                    data_idx += partition.0 as u32;

                    partitions.push_back((partition.0, 0, false));
                    partitions.swap_remove_back(idx);

                    if rest != 0 {
                        partitions.push_front((rest, block_idx, is_data));
                    }
                }
                _ => data_idx += len as u32,
            }
        }
    }
    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
