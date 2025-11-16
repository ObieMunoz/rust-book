fn main() {
    for num in 1..=15 {
        println!("{}", fib(num));
    }
}

fn fib(num: i32) -> i32 {
    if num == 0 { return 0 };
    if num <= 2 { return 1 };
    fib(num - 1) + fib(num - 2)
}
