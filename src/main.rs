// This was created by cargo init
// Maybe I need to try cargo new instead
fn main() {
    println!("Welcome to the teXt editor 🔥🦀!");
    println!("This is a program I wrote to learn about Rust\n\n");
    let mut input = String::new();
    while std::io::stdin().read_line(&mut input).unwrap() > 0 {
        // println!("\n\n----------\n{}\n---------\n", input);  
    }
}
