use std::{io, usize};

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

    // let s1 = String::from("New String!");
    // let s2 = s1;
    // println!("{s2}");
    let l = cal_len(&s);
    println!("Length of  string: {s} = {l}");

    add_string(&mut s);
    println!("New string: {s}");
}

fn cal_len(s: &String) -> usize {
    s.len()
}

fn add_string(s: &mut String) {
    s.push_str(", hello!")
}
