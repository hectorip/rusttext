// This was created by cargo init

fn main() {
    println!("Welcome to the teXT editor 🔥🦀!");

    // println!("This is a program I wrote to learn about Rust\n\n");

    let mut input = String::new();
    while std::io::stdin().read_line(&mut input).unwrap() > 0 {
        if input.trim() == "q" {
            println!("Salir");
            input.clear();
            break;

        } else {
            input.clear();
        }
    }
}
