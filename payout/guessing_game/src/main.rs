use std::io::{self, BufRead};


fn main() {
    let stdin = io::stdin();

    println!("Pres Ctrl+D / Ctrl+Z to stop: ");
    for line in stdin.lock().lines() {
        match line {
            Ok(text) => println!("typed: {text}"),
            Err(err) => eprintln!("error reading line {err}"),
        }
    }
    println!("Hello, world!");
}
