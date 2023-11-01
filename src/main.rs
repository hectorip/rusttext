// This was created by cargo init

use std::io::Read;

fn main() {
    println!("Welcome to the teXT editor 🔥🦀!");

    // println!("This is a program I wrote to learn about Rust\n\n");

    let mut input: []] = String::new();
    while std::io::stdin().read_exact(&mut input).unwrap() > 0 {
        if input.trim() == "q" {
            println!("Salir");
            input.clear();
            break;
        } else {
            input.clear();
        }
    }
}
