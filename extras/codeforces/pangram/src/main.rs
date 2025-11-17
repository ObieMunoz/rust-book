use std::{io::{self, BufRead}, process::exit};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let length: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse number of test cases");

    if length < 26 {
        println!("NO");
        exit(0);
    }

    let line = lines.next().unwrap().unwrap().to_lowercase();
    let mut char_code_vec = Vec::new(); 

    for c in line.chars() {
       char_code_vec.push(c as u32);
    }
    
    // println!("{},{}", 'a' as u32, 'z' as u32);
    // 97 - a 
    // 122 - z

    for i in 97..=122 {
        if char_code_vec.contains(&i) {
            continue;
        } else {
            println!("NO");
            exit(0);
        }
    }
    println!("YES");
}

