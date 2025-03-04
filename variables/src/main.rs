use std::io;

fn main() {
    let mut x = 10;
    println!("{x}");

    x = 5;

    println!("x:{}", x);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    let spaces = "  ";
    let spaces = spaces.len();
    println!("{spaces}");

    let guess: u32 = "42".parse().expect("Expected int value");
    let short: u32 = 42_000;
    println!("{short}");
    println!("{guess}");

    let tup = (500, 6.4, 1);

    let y = tup.1;

    println!("The value of y is: {y}");

    println!("Enter the index number: ");

    let a = [3; 5];
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Expected a number ");

    let index: usize = index.trim().parse().expect("Expected a number");

    let el = a[index];

    println!("Element at {index} = {el}");
}
