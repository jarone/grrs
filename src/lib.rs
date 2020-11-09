use std::{error::Error, fs::File, io::prelude::*, io::BufReader, process};

pub fn run(raw_args: &[String]) -> Result<(), Box<dyn Error>> {
    let args = Args::new(&raw_args).unwrap_or_else(|err| {
        println!("Parsing arguments failed: {}", err);
        process::exit(1);
    });

    let file = File::open(args.path)?;
    let file = BufReader::new(file);

    match_line(file, args.pattern);

    Ok(())
}

struct Args<'a> {
    pattern: &'a String,
    path: &'a String,
}

impl<'a> Args<'a> {
    fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Args {
            pattern: &args[1],
            path: &args[2],
        })
    }
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
