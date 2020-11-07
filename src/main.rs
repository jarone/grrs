use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    search(&args.path, &args.pattern);

    Ok(())
}

fn search(path: &std::path::PathBuf, pattern: &String) {
    let file = File::open(path)
        .expect(&(String::from("could not open the file: ") + &path.display().to_string()));
    let file = BufReader::new(file);

    match_line(file, pattern);
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
