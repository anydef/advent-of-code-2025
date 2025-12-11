use std::fs::read_to_string;

fn main() {
    println!("Day 06");
    read_task("resources/day06/example_input.txt")
    // read_task("resources/day06/input.txt")
}

fn read_task(filename: &str) -> () {
    let content = read_to_string(filename).unwrap();
    let lines: Vec<Vec<&str>> = content
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|part| {
                    let x1 = part.trim();
                    x1
                })
                .filter(|x| !x.is_empty())
                .collect()
        })
        .collect();
    //
    let results: Vec<i64> = Vec::new();
    //
    let cnt = lines.len();
    //
    let op_line = lines.last().unwrap();
    let mut t: Vec<(&str, Vec<&str>)> = Vec::new();
    for (idx, op) in op_line.iter().enumerate() {
        let mut task = Vec::new();
        let nums_vec = lines.iter().take(cnt - 1);
        for nums in nums_vec {
            let num = nums[idx];
            let result = num.parse::<i64>().expect("failed to parse string");
            task.push(num);
        }

        t.push((op, task));
        // let res = match *op {
        //     "*" => task
        //         .iter()
        //         .copied()
        //         .reduce(|a, b| a * b)
        //         .expect("Could not reduce task"),
        //     "+" => task
        //         .iter()
        //         .copied()
        //         .reduce(|a, b| a + b)
        //         .expect("Could not reduce task"),
        //     _ => {
        //         panic!("Unknown operator")
        //     }
        // };

        // results.push(res)
    }

    for (op, nums) in t {
        let max_len = nums.iter().map(|s| s.len()).max().unwrap();
        let padded: Vec<Vec<char>> = nums.iter()
            .map(|s| {
                format!("{:x>width$}", s, width = max_len)
            })
            .map(|s| {
                s.chars().collect()
            })
            .collect();
        let mut transposed: Vec<i64> = Vec::new();
        for i in 0..max_len {
            let mut num_ch: Vec<char> = Vec::new();
            for num in &padded {
                let c = num[i];
                if c == 'x' {
                    continue;
                }
                num_ch.push(c);
            }
            let x2: String = num_ch.iter().collect();
            transposed.push(x2.parse::<i64>().expect("failed to parse string"))
        };

        println!("num {:?}", transposed);

    }

    // let option = results
    //     .iter()
    //     .copied()
    //     .reduce(|x4, x5| x4 + x5)
    //     .expect("Could not reduce task");
    //
    // println!("Result: {}", option);

    // lines;
    ()
    // lines
    //     .map(String::from)
    //     // .filter(move |x| x.contains("-"))
    //     .map(move |x| {
    //         let split = x.split_once("-");
    //         match split {
    //             None => panic!("Invalid range format"),
    //             Some((l, r)) => {
    //                 let left_num = l.parse::<u64>().expect("Invalid number format");
    //                 let right_num = r.parse::<u64>().expect("Invalid number format");
    //                 Range {from:left_num, to:right_num}
    //             }
    //         }
    //     })
    //     .collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_nothing() {
        assert_eq!("00000110", format!("{:0<8}", "110"));
    }
    #[test]
    fn test_nothing2() {
        // assert_eq!("00000110", format!("{:0<8}", "110"));
        let title = "638";
        // println!("{:0<width$}", title, width = 5);
        let string = format!("{:0<width$}", "110", width = 5);
        println!("{string}")
    }
}
