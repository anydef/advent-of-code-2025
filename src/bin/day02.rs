use std::fs::read_to_string;

fn main() {
    print!("Day 02\n");
    let vec = read_ranges("resources/day02/input.txt");
    // println!("{:?}", vec);
    let mut sum_invalid_ids = 0;
    for (start, end) in vec {
        // println!("Range: {} - {}", start, end);
        for id in start..=end + 1 {
            let id_str = id.to_string();
            let i = id_str.len();
            if i % 2 != 0 {
                // if number of digits is odd, skip
                continue;
            }
            let (first, rest) = id_str.split_at(i / 2);
            if first != rest {
                continue;
            }
            println!("invalid id detected: {}", id);
            sum_invalid_ids += id;
            // println!("{}", id);
        }
    }
    println!("Result: {}", sum_invalid_ids);
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
    use crate::read_ranges;

    #[test]
    fn test_parsing() {
        let vec = read_ranges("resources/day02/input.txt");
        println!("{:?}", vec);
    }
}
