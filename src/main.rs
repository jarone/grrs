use std::{fs::File, io::prelude::*, io::BufReader};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let file = File::open(&args.path);
    let file = match file {
        Ok(file) => BufReader::new(file),
        Err(why) => panic!("could not open file: {}", why),
    };

    match_line(file, &args.pattern);
}

fn match_line(file: BufReader<File>, pattern: &String) {
    let mut line_number = 0;

    for line in file.lines() {
        let line_string = line.unwrap();
        line_number += 1;
        if line_string.contains(pattern) {
            println!("{} {}", line_number, line_string);
        }
    }
}
