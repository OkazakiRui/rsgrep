use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rsgrep", about = "Rust Grep command tool")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: Vec<String>,
}

fn grep(content: String, grep_args: &GrepArgs, file_name: &str) {
    for line in content.lines() {
        if line.contains(grep_args.pattern.as_str()) {
            println!("{}: {}", file_name, line);
        }
    }
}

fn run(grep_args: GrepArgs) {
    for file in grep_args.path.iter() {
        match read_to_string(file) {
            Ok(contents) => grep(contents, &grep_args, file),
            Err(e) => println!("{}", e),
        }
    }
}

fn main() {
    run(GrepArgs::from_args());
}
