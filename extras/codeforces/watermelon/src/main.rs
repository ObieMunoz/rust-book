use std::io;

fn main() {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    match number.trim().parse() {
        Ok(num) => {
            if num % 2 == 0 && num > 2 {
                println!("YES");
                num
            } else {
                println!("NO");
                num
            }
        },
        Err(_) => 0,
    };
}
