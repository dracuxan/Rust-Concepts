use std::io;

fn main() {
    let mut s = {
        let mut user_input = String::new();
        println!("Enter your name:");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Expected a string input.");
        user_input
    };

    s = s.trim().to_string();

    println!("Hello {s}!");

    let s1 = String::from("New String!");
    let s2 = s1;
    println!("{s2}");
}
