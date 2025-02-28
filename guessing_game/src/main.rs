use std::io;

fn main() {
    println!("Wlecome to Guess The Number! Enter a guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read lines");

    println!("Your guess: {}", guess);
}
