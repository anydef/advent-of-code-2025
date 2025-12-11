use std::fs::read_to_string;
use std::str::Chars;

#[derive(Debug)]
struct Task {
    nums: Vec<u64>,
    op: char,
}

impl Task {
    fn op(&self) -> u64 {
        match self.op {
            '*' => self.nums.iter().copied().reduce(|a, b| a * b).unwrap(),
            '+' => self.nums.iter().copied().reduce(|a, b| a + b).unwrap(),
            _ => 0
        }
    }
}

fn main() {
    println!("Day 06");
    // let lines = read_lines("resources/day06/example_input.txt");
    let lines = read_lines("resources/day06/input.txt");
    let row_cnt = lines.len();
    let row_max_len = lines
        .iter()
        .take(row_cnt - 1)
        .map(|x| x.len())
        .max()
        .unwrap();
    let mut op_line = lines.iter().last().unwrap().chars().clone();
    println!("{:?}", op_line);
    let mut num_lines: Vec<Chars> = Vec::new();
    for line in lines.iter().take(row_cnt - 1) {
        num_lines.push(line.chars())
    }
    num_lines.len();

    let mut current_op: char;
    let mut tasks: Vec<Task> = Vec::new();
    let mut current_task: Task = Task {
        nums: Vec::new(),
        op: '-',
    };
    const RADIX: u32 = 10;
    for i in 0..row_max_len {
        let mut current_number_vec: Vec<u32> = Vec::new();

        if let Some(op_char) = op_line.next() {
            match op_char {
                '*' | '+' => {
                    current_op = op_char;
                    current_task = Task {
                        nums: Vec::new(),
                        op: current_op,
                    };
                    tasks.push(current_task);
                    println!("OP {current_op}");
                }
                _ => {}
            }
        }

        let mut current_number: u64 = 0;
        for (idx, line) in num_lines.iter().enumerate() {
            if let Some(c) = line.clone().nth(i)
                && c.is_numeric()
            {
                let digit = c.to_digit(RADIX).expect("Not a digit");
                current_number_vec.push(digit);
            }
        }
        let l = current_number_vec.len();
        for (i, n) in current_number_vec.iter().enumerate() {
            let pow: u64 = (10u64).pow((l - i - 1) as u32);
            current_number += (*n as u64) * pow
        }
        if let Some(ct) = tasks.last_mut()
            && current_number > 0
        {
            ct.nums.push(current_number);
        }
        println!("number: {current_number}");
        println!("---")
    }
    let res:u64 = tasks.iter().map(|x| { x.op() }).reduce(|a, b| { a + b }).unwrap() as u64;
    println!("Res: {res}");
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_nothing() {
        assert_eq!("00000110", format!("{:0<8}", "110"));
    }
    #[test]
    fn test_nothing2() {
        let title = "638";
        let string = format!("{:0<width$}", "110", width = 5);
        println!("{string}")
    }
}
