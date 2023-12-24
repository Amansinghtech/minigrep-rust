use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filepath = &args[2];

    println!("Searching for: \"{query}\"");
    println!("In File: {filepath}");

    let content = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");

    println!("file content: \n{content}")
}
