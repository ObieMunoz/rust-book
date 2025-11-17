use std::io::{self, BufRead};

fn main() {
    /*
    let mut num_test_cases = String::new();

    io::stdin()
        .read_line(&mut num_test_cases)
        .expect("Failed to read line");

    match num_test_cases.trim().parse::<i32>() {
        Ok(num) => {
            for _i in 0..num {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                let mut numbers = input.split_whitespace().map(|s| s.parse::<i32>());

                let first = numbers
                    .next()
                    .expect("First number missing")
                    .expect("First number invalid");
                let second = numbers
                    .next()
                    .expect("Second number missing")
                    .expect("Second number invalid");

                find_next_divisible_number(first, second);
            }
        }
        Err(_) => {}
    }
    */

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let num_test_cases: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse number of test cases");

    for _ in 0..num_test_cases {
        let line = lines.next().unwrap().unwrap();
        let mut numbers = line.split_whitespace();

        let a: i32 = numbers.next().unwrap().parse().unwrap();
        let b: i32 = numbers.next().unwrap().parse().unwrap();

        find_next_divisible_number(a, b);
    }
}

fn find_next_divisible_number(a: i32, b: i32) {
    let remainder = a % b;
    let count = if remainder == 0 { 0 } else { b - remainder };
    println!("{}", count);
}
