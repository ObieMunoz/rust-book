use std::io;

fn main() {
    loop {
        println!("Enter a temperature in fahrenheit:");
        let mut temperature_fahrenheit = String::new();

        io::stdin()
            .read_line(&mut temperature_fahrenheit)
            .expect("Failed to read line");

        match temperature_fahrenheit.trim().parse::<i32>() {
            Ok(num) => {
                println!("{temperature_fahrenheit} Fahrehneit to Celcius is: {}", fahrenheit_to_celcius(num));
                break
            },
            Err(_) => continue,
        };
    }
}


fn fahrenheit_to_celcius(num: i32) -> i32 {
    (num - 32) * 5 / 9
}
