fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!!!");
    forloop();
    forrev();
}

fn forloop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    println!("-------------------");

    for element in a {
        println!("The value is: {element}");
    }
}

fn forrev() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!!!");
}
