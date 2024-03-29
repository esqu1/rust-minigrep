use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let filename: &String = &args[2];

    println!("Command: {}", query);
    println!("Filename: {}", filename);
}
