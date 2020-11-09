use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(err) = grrs::run(&args) {
        println!("Error: {}", err);
        process::exit(1);
    }
}
