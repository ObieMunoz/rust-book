use std::io;

fn main() {
    loop {
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        let word = word.trim();
        if word.is_empty() {
            break;
        };

        let number = word.parse::<u32>();
        match number {
            Ok(_ok) => continue,
            Err(_e) => {}
        }

        let len = word.len();
        let first = word.chars().next().unwrap();
        let last = word.chars().last().unwrap();

        if len > 10 {
            println!("{}{}{}", first, len - 2, last);
        } else {
            println!("{}", word);
        }
    }
}
