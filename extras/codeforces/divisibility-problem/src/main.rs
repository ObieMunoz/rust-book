use std::io;

fn main() {
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
}

fn find_next_divisible_number(a: i32, b: i32) {
    let remainder = a % b;
    let count = if remainder == 0 { 0 } else { b - remainder };
    println!("{}", count);
}
