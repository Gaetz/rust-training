#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Problem in reading the input.");
    println!("Hello {}, how are you?", name);
}
