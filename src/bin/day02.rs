use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    print!("Day 02\n");
    let vec = read_ranges("resources/day02/input.txt");

    let sum_invalid_ids = sum_invalid_ids(vec);
    println!("Result: {}", sum_invalid_ids);
}

fn sum_invalid_ids(vec: Vec<(i64, i64)>) -> i64 {
    let mut sum_invalid_ids = 0;
    for (start, end) in vec {
        for id in start..=end {
            let valid = check_id_valid(id);
            if valid {
                continue;
            }
            // println!("Invalid ID: {}", id);
            sum_invalid_ids += id;
        }
    }
    sum_invalid_ids
}

fn check_id_valid(id: i64) -> bool {
    let mut valid = true;
    let part_sizes = possible_part_sizes(id);
    for part_size in part_sizes {
        if !valid {
            break;
        }
        let id_str = id.to_string();
        let mut parts = split_parts(id_str.as_str(), part_size);
        let unique_parts: HashSet<&str> = parts.iter().copied().collect();
        let i = unique_parts.len();
        if i == 1 {
            valid = false;
        }

    }
    valid
}

fn possible_part_sizes(id: i64) -> Vec<usize> {
    let id_str = id.to_string();
    let id_len = id_str.len();
    // loop over possible part lengths
    let mut part_sizes: Vec<usize> = vec![];
    for part_len in (1..=id_len / 2) {
        if id_len % part_len != 0 {
            continue;
        }
        part_sizes.push(part_len);
    }
    part_sizes
}

fn split_parts(test_str: &str, step: usize) -> Vec<&str> {
    let mut from = 0;
    let mut parts: Vec<&str> = vec![];
    for i in (0..test_str.len() + 1).step_by(step) {
        if i == from {
            continue;
        }

        // println!("From {from}, index {i}");
        let x = &test_str[from..i];
        parts.push(x);
        // println!("Part: {}", x);

        from = i;
    }
    parts
}

fn read_ranges(filename: &str) -> Vec<(i64, i64)> {
    let v: Vec<(i64, i64)> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .flat_map(|line| {
            let ranges_split = line.split(",");
            let mut ranges: Vec<String> = vec![];
            for part in ranges_split {
                ranges.push(part.to_string());
            }
            ranges
        })
        .map(|range| {
            let r = range.split_once("-");
            match r {
                None => panic!("Invalid range format!"),
                Some((left, right)) => {
                    let left_num = left.parse::<i64>().expect("Invalid number format!");
                    let right_num = right.parse::<i64>().expect("Invalid number format!");
                    (left_num, right_num)
                }
            }
        })
        .collect();
    v
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parsing() {
        let vec = read_ranges("resources/day02/input.txt");

        println!("{:?}", vec);
    }

    #[test]
    fn test_ranges() {
        assert_eq!(sum_invalid_ids(vec![(11, 22)]), 11 + 22);
        assert_eq!(sum_invalid_ids(vec![(95, 115)]), 99 + 111);
        assert_eq!(sum_invalid_ids(vec![(998, 1012)]), 999 + 1010);
        assert_eq!(sum_invalid_ids(vec![(1188511880, 1188511890)]), 1188511885);
        assert_eq!(sum_invalid_ids(vec![(565653, 565659)]), 565656);
    }

    #[test]
    fn test_check_id_valid() {
        assert_eq!(check_id_valid(123123), false);
        assert_eq!(check_id_valid(123456), true);
        assert_eq!(check_id_valid(1212), false);
        assert_eq!(check_id_valid(1234), true);
        assert_eq!(check_id_valid(2121212121), false);
        assert_eq!(check_id_valid(38593859), false);
        assert_eq!(check_id_valid(038593859), false);
        assert_eq!(check_id_valid(555554), true);
        assert_eq!(check_id_valid(74374363), true);
    }

    #[test]
    fn test_index_split() {
        let test_str = "123456";
        let vec = split_parts(test_str, 2);
        assert_eq!(vec, vec!["12", "34", "56"]);
    }

    #[test]
    fn test_part_len_iter() {
        let vec = possible_part_sizes(1234567890);
        assert_eq!(vec, vec![2, 3, 4, 5])
    }
}
