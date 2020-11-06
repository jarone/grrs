use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {}", &args.path.display()))?;

    match_line(&content, &args.pattern);

    Ok(())
}

fn match_line(content: &String, pattern: &String) {
    let mut line_number = 1;

    for line in content.lines() {
        if line.contains(pattern) {
            println!("{} {}", line_number, line);
        }
        line_number += 1;
    }
}
