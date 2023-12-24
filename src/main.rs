use std::{env, fs, process};
use lib::Config;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config= Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: \"{}\"", config.query);
    println!("In File: {}", config.filepath);

    let content = fs::read_to_string(config.filepath)
        .expect("Should have been able to read the file");

    println!("file content: \n{content}")
}

/* [Deprecated]
fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let filepath = &args[2];

    Config { query, filepath }
}
*/
