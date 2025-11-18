use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let dollars: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse dollars");

    let mut notes = 0;
    let mut current: u32 = dollars as u32;

    let hundreds = current / 100;
    current -= hundreds * 100;
    let twenties = current / 20;
    current -= twenties * 20;
    let tens = current / 10;
    current -= tens * 10;
    let fives = current / 5;
    current -= fives * 5;
    let ones = current / 1;
    notes += hundreds + twenties + tens + fives + ones;
    println!("{}", notes);
}
