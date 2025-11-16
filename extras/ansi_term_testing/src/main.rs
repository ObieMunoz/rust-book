use ansi_term::Color;
use ansi_term::Style;

fn main() {
    println!("This is {} in color, {} in color, and {} in color",
        Color::Red.paint("red"),
        Color::Blue.paint("blue"),
        Color::Green.paint("green"));

    println!("{} and this is not", Style::new().bold().paint("This is Bold"));
    println!("{}", Color::Red.bold().paint("this is bold and colored"));
}
