fn main() {
    let mut x = 10;
    println!("{x}");
    x = 5;
    println!("x:{}", x);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}")
}
