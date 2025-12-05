use std::fs::read_to_string;
const CELLS: usize = 12;

fn main() {
    let vec = read_batteries_from_file("resources/day03/input.txt");
    let mut sum: u128 = 0;
    for b in &vec {
        let max_capacity_batteries = find_max_capacity_v2(b, CELLS);
        let capacity = char_to_capacity_v2(&max_capacity_batteries);
        println!("max cap for {:?} {}", max_capacity_batteries, capacity);
        sum += capacity
    }

    println!("Sum of max capacities: {}", sum);
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

fn find_largest_battery(bank: &Bank, start_idx: Pos) -> (Pos, Battery) {
    let mut largest: Battery = '0';
    let mut pos: usize = 0;

    let end = if start_idx == 0 { bank.len() - 1 } else { bank.len() };
    for (i, &battery) in bank[start_idx..end].iter().enumerate() {
        if battery > largest {
            largest = battery;
            pos = i;
        }
    }
    (pos, largest)
}

fn find_largest_battery_v2(bank: &Bank, curr_battery: usize, start_idx: Pos, cells: usize) -> (Pos, Battery) {
    let mut largest: Battery = '0';
    let mut pos: usize = 0;
    let end_index = bank.len() - (cells - curr_battery);
    for i in start_idx..end_index {
        let battery = &bank[i];
        if battery > &largest {
            largest = *battery;
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

fn find_max_capacity_v2(bank: &Bank, cells: usize) -> Vec<Battery> {
    let mut batteries: Vec<Battery> = Vec::with_capacity(cells);
    let mut start_idx: usize = 0;
    for i in 0..cells {
        // last_idx = i;
        let (pos, battery) = find_largest_battery_v2(bank, i+1, start_idx, cells);
        start_idx = pos + 1;
        batteries.push(battery)
    }
    batteries
}

fn char_to_capacity((b1, b2): (Battery, Battery)) -> u32 {
    const RADIX: u32 = 10;
    b1.to_digit(RADIX).unwrap() * 10 + b2.to_digit(RADIX).unwrap()
}

fn char_to_capacity_v2(bank: &Bank) -> u128 {
    const RADIX: u32 = 10;
    let mut sum: u128 = 0;
    for (pos, battery) in bank.iter().enumerate() {
        sum += battery.to_digit(RADIX).unwrap() as u128
            * 10u128.pow((bank.len() - pos - 1) as u32);
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    fn test_char_to_capacity_v2() {
        let capacity = char_to_capacity_v2(&vec!['4', '3']);
        assert_eq!(capacity, 43);

        let capacity = char_to_capacity_v2(&vec!['1', '0']);
        assert_eq!(capacity, 10);

        let capacity = char_to_capacity_v2(&vec!['9', '8', '7', '6', '5', '4', '3', '2', '1', '1', '1', '1']);
        assert_eq!(capacity, 987654321111);
    }
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

    #[test]
    fn test_dozen_batteries() {
        let vec = read_batteries_from_file("resources/day03/example_input.txt");
        let capacity = find_max_capacity_v2(&(&vec[0]), CELLS);
        assert_eq!(capacity, vec!['9', '8', '7', '6', '5', '4', '3', '2', '1', '1', '1', '1']);

        let capacity = find_max_capacity_v2(&(&vec[1]), CELLS);
        assert_eq!(capacity, vec!['8', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '9']);

        let capacity = find_max_capacity_v2(&(&vec[2]), CELLS);
        assert_eq!(capacity, vec!['4','3','4','2','3','4','2','3','4','2','7','8']);

        let capacity = find_max_capacity_v2(&(&vec[3]), CELLS);
        assert_eq!(capacity, vec!['8','8','8','9','1','1','1','1','2','1','1','1']);
    }

    #[test]
    fn test_max_battery() {
        let vec = read_batteries_from_file("resources/day03/example_input.txt");
        let bank = &vec[0];
        let (pos, battery) = find_largest_battery_v2(&bank, 1, 0, CELLS);
        assert_eq!(pos, 0);
        assert_eq!(battery, '9');

        let (pos, battery) = find_largest_battery_v2(&bank, 2, pos+1, CELLS);
        assert_eq!(pos, 1);
        assert_eq!(battery, '8');

        let (pos, battery) = find_largest_battery_v2(&bank, 11, pos+10, CELLS);
        assert_eq!(pos, 11);
        assert_eq!(battery, '1');

    }

    #[test]
    fn test_indexing() {
        vec![1, 2, 3, 4, 5, 6];

        for i in 4..6 {
            let slice = &vec![1, 2, 3, 4, 5, 6][i..];
            println!("i: {}, slice: {:?}", i, slice);
        }
    }


}
