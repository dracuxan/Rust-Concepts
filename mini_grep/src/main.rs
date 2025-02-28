use std::env;
use std::process;

use mini_grep::Config;

fn main() {
    println!("Welcome to Rusty Grep!");
    let args: Vec<String> = env::args().collect();
    let config = Config::create(&args).unwrap_or_else(|err| {
        println!("Problem Parsing the arguments: {err}");
        process::exit(1);
    });

    println!("Search query: {}", config.query);
    println!("File to search: {}", config.file_path);

    if let Err(e) = mini_grep::run(config) {
        println!("Application encountered and error while executing: {e}");
        process::exit(1);
    }
}
