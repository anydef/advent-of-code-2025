use std::cmp;
use std::fs::read_to_string;

fn main() {
    let (mut ranges, ids) = read_db("resources/day05/input.txt");
    // let (ranges, ids) = read_db("resources/day05/example_input.txt");

    let fresh_ids = count_fresh_ids(&ranges, ids);
    println!("Spoiled ids: {}", fresh_ids);

    let vec = reduce_ranges_max(&mut ranges);
    let all_ids = count_all_ids_in(&vec);
    println!("All ids: {all_ids}");
}

type Id = u64;
#[derive(Debug, Clone)]
struct Range {
    from: u64,
    to: u64,
}

impl Range {
    fn count(&self) -> usize {
        ((self.to - self.from) + 1) as usize
    }
}

trait Checker {
    fn in_range(&self, id: Id) -> bool;
}
impl Checker for Range {
    fn in_range(&self, id: Id) -> bool {
        id >= self.from && id <= self.to
    }
}

fn count_all_ids_in(ranges: &Vec<Range>) -> usize {
    let mut count: usize = 0;
    for r in ranges {
        count += r.count()
    }
    count
}

fn reduce_ranges_max(ranges: &mut Vec<Range>) -> Vec<Range> {
    ranges.sort_by(|x, x1| {x.from.cmp(&x1.from)});
    let mut result: Vec<Range> = Vec::new();

    result.push(ranges[0].clone());

    for i in 1..ranges.len() {
        let current = ranges[i].clone();
        let last_in_result_idx = result.len()-1;
        if current.from >= result[last_in_result_idx].from && current.from  <= result[last_in_result_idx].to {
            result[last_in_result_idx].to = cmp::max(current.to, result[last_in_result_idx].to);
        } else {
            result.push(current);
        }

    }
    result
}


fn count_fresh_ids(ranges: &Vec<Range>, ids: Vec<Id>) -> usize {
    only_fresh_ids(ranges, ids).len()
}

fn only_fresh_ids(ranges: &Vec<Range>, ids: Vec<Id>) -> Vec<Id> {
    let mut fresh: Vec<Id> = Vec::new();
    'id_loop: for id in ids {
        for range in ranges {
            if range.in_range(id) {
                fresh.push(id);
                continue 'id_loop;
            }
        }
    }
    fresh
}

fn read_db(filename: &str) -> (Vec<Range>, Vec<Id>) {
    let ranges: Vec<Range> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .filter(move |x| x.contains("-"))
        .map(move |x| {
            let split = x.split_once("-");
            match split {
                None => panic!("Invalid range format"),
                Some((l, r)) => {
                    let left_num = l.parse::<u64>().expect("Invalid number format");
                    let right_num = r.parse::<u64>().expect("Invalid number format");
                    Range {from:left_num, to:right_num}
                }
            }
        })
        .collect();
    let ids: Vec<Id> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .filter(move |x| !x.contains("-") && !x.is_empty())
        .map(move |x| x.parse::<u64>().expect("Invalid number format"))
        .collect();
    (ranges, ids)
}

#[cfg(test)]
mod test {
    use crate::{count_all_ids_in, only_fresh_ids, read_db, reduce_ranges_max, Id};

    #[test]
    fn test_parse_file() {
        let db = read_db("resources/day05/example_input.txt");
        println!("{:?}", db);
        assert_eq!(1, 1)
    }

    #[test]
    fn test_check_ids_legit() {
        let (range_vec, ids) = read_db("resources/day05/example_input.txt");

        let fresh_ids = only_fresh_ids(&range_vec, ids);

        assert!(fresh_ids.contains(&(5 as Id)));
        assert!(fresh_ids.contains(&(11 as Id)));
        assert!(fresh_ids.contains(&(17 as Id)));

        assert!(!fresh_ids.contains(&(1 as Id)));
        assert!(!fresh_ids.contains(&(8 as Id)));
        assert!(!fresh_ids.contains(&(32 as Id)));

    }

    #[test]
    fn test_reduce_ranges_max() {
        let (mut range_vec, _) = read_db("resources/day05/example_input.txt");
        assert_eq!(range_vec.len(), 4);
        let vec = reduce_ranges_max(&mut range_vec);
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_ranges_ids_count() {
        let (mut range_vec, _) = read_db("resources/day05/example_input.txt");
        assert_eq!(range_vec.len(), 4);
        let vec = reduce_ranges_max(&mut range_vec);
        assert_eq!(vec.len(), 2);
        let all_ids = count_all_ids_in(&vec);
        assert_eq!(all_ids, 14);
    }

    #[test]
    fn test_range() {
        for i in 1..=5 {
            println!("{i}");
        }
    }
}
