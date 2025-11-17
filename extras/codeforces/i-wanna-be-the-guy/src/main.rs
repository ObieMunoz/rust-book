use std::io::{self, BufRead};
use std::process::exit;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let levels: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse number of test cases");

    let line1 = lines.next().unwrap().unwrap();
    let little_x_levels = line1.split_whitespace().into_iter().skip(1); // ignore the first element
    // which is the count of levels that can be beaten

    let line2 = lines.next().unwrap().unwrap();
    let little_y_levels = line2.split_whitespace().into_iter().skip(1);

    let mut all_completable_levels = Vec::new();

    for i in little_y_levels {
        all_completable_levels.push(i.parse::<i32>().unwrap());
    }

    for i in little_x_levels {
        all_completable_levels.push(i.parse::<i32>().unwrap());
    }

    // println!("{:?}", all_completable_levels);

    for i in 1..=levels {
        let level: i32 = i as i32;
        if !all_completable_levels.contains(&level) {
            println!("Oh, my keyboard!");
            exit(0);
        }
    }

    println!("I become the guy.");
}
