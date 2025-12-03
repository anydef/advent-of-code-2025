use std::fs::read_to_string;
const START_KNOB: i32 = 50;

fn main() {
    print!("Day 01\n");
    let vec = read_codes_from_file("resources/day01/input.txt");
    let mut current_value = START_KNOB;
    let mut zero_counter = 0;
    for code in vec.iter() {
        if current_value % 100 == 0 {
            zero_counter = zero_counter + 1;
        }
        current_value = current_value + code;
    }

    print!("Result: {:?}", zero_counter);
    ()
}

fn read_codes_from_file(filename: &str) -> Vec<i32> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|s| {
            let (first, rest) = s.trim().split_at(1);
            (first.to_string(), rest.to_string())
            // x.split_at(1)
        })
        .map(|(first, rest)| {
            let multiplier = if first == "R" {
                -1
            } else if first == "L" {
                1
            } else {
                panic!("Invalid starting character!")
            };
            let result = rest.parse::<i32>().expect("Invalid number format!");

            result * multiplier
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mod() {
        assert_eq!(0 % 100, 0);
    }
}
