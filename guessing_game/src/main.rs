use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_key = rand::rng().random_range(1..=100);
    println!("Secret Number: {secret_key}");
    println!("Wlecome to Guess The Number! Enter a guess:");

    loop {
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read lines");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_key) {
            Ordering::Less => println!("Too Less!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
        }
    }
}
