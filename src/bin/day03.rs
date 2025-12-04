use std::fs::read_to_string;

fn main() {
    let vec = read_batteries_from_file("resources/day03/input.txt");
    let mut sum: u32 = 0;
    for b in &vec {
        let max_capacity_batteries = find_max_capacity(b);
        let capacity = char_to_capacity(max_capacity_batteries);
        println!("max cap for {:?} {}", max_capacity_batteries, capacity);
        sum += capacity
    }

    println!("Sum of max capacities: {}", sum);
    // let max_joltage: u32 = vec
    //     .iter()
    //     .map(|b| {
    //         let max_capacity_batteries = find_max_capacity(b);
    //         let capacity = char_to_capacity(max_capacity_batteries);
    //         println!("max cap for {:?} {}", max_capacity_batteries, capacity);
    //         capacity
    //     })
    //     .sum();
    // println!("Max joltage: {max_joltage}");
}

type Battery = char;
type Pos = usize;
type Bank = Vec<Battery>;

fn read_batteries_from_file(filename: &str) -> Vec<Bank> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|s| s.chars().map(|c| c).collect::<Bank>())
        .collect()
}

/// First largest battery in the bank, excluding the last one.
fn find_largest_battery(bank: &Bank, start_idx: Pos) -> (Pos, Battery) {
    let mut largest: Battery = '0';
    let mut pos: usize = 0;

    let end = if start_idx == 0 { bank.len() - 1 } else { bank.len() };
    // let start = if start_idx == 0 { 0 } else { start_idx + 1 };
    for (i, &battery) in bank[start_idx..end].iter().enumerate() {
        if battery > largest {
            largest = battery;
            pos = i;
        }
    }
    (pos, largest)
}

fn find_max_capacity(bank: &Bank) -> (Battery, Battery) {
    let (first_pos, first_battery) = find_largest_battery(bank, 0);
    let (second_pos, second_battery) = find_largest_battery(bank, first_pos+1);
    (first_battery, second_battery)
}

fn char_to_capacity((b1, b2): (Battery, Battery)) -> u32 {
    const RADIX: u32 = 10;
    b1.to_digit(RADIX).unwrap() * 10 + b2.to_digit(RADIX).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_char_comp() {
        let c1: char = '0';
        let c2: char = '1';
        assert!(c1 < c2);
    }

    #[test]
    fn test_load_file() {
        let vec = read_batteries_from_file("resources/day03/input.txt");
        assert_eq!(vec[0][0], '4');
    }

    #[test]
    fn test_char_to_capacity() {
        let capacity = char_to_capacity(('4', '3'));
        assert_eq!(capacity, 43);

        let capacity = char_to_capacity(('1', '0'));
        assert_eq!(capacity, 10);
    }

    #[test]
    fn test_find_max_capacity() {
        let bank: Bank = vec!['1', '4', '3', '2'];
        let capacity = find_max_capacity(&bank);
        assert_eq!(capacity, ('4', '3'));


        let bank: Bank = vec!['1', '4', '8', '9'];
        let capacity = find_max_capacity(&bank);
        assert_eq!(capacity, ('8', '9'));
    }

    #[test]
    fn test_find_largest_battery() {
        let (pos, battery) = find_largest_battery(&vec!['1', '4', '3', '2'], 0);

        assert_eq!(pos, 1);
        assert_eq!(battery, '4');

        let (pos, battery) = find_largest_battery(&vec!['1', '4', '3', '5'], 0);

        assert_eq!(pos, 1);
        assert_eq!(battery, '4');

        let (pos, battery) = find_largest_battery(&vec!['6', '4', '3', '5'], 0);

        assert_eq!(pos, 0);
        assert_eq!(battery, '6');
    }

    #[test]
    fn test_task_example() {
        let vec = read_batteries_from_file("resources/day03/example_input.txt");
        let mut sum: u32 = 0;
        for b in &vec {
            let max_capacity_batteries = find_max_capacity(b);
            let capacity = char_to_capacity(max_capacity_batteries);
            println!("max cap for {:?} {}", max_capacity_batteries, capacity);
            sum += capacity
        }

        assert_eq!(sum, 357);
    }


}
