/*
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}
*/

fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}

fn main() {
    return_a_string();
}
