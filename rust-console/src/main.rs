use std::io;

fn main() {
    let separator: String = String::from("-------------");
    let mut choice = String::new();
    let doContinue: bool = false;
    println!("{} Rust console {}", separator, separator);
    println!("1. Guessing game");
    println!("2. Variables and Mutability");
    println!("{separator}");
    println!("Enter your choice: ");
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(number) => number,
        Err(_) => 0,
    };

    match choice {
        1 => {
            println!("Selected guessing game...");
        },
        _ => {
            println!("Invalid input...")
        }
    }
}
