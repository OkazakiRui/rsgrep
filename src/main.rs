use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rsgrep", about = "Rust Grep command tool")]
struct GrepArgs {
    #[structopt(name = "FILE")]
    path: String,
    #[structopt(name = "PATTERN")]
    pattern: String,
}

fn grep(content: String, grep_args: &GrepArgs) {
    for line in content.lines() {
        if line.contains(grep_args.pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn run(grep_args: GrepArgs) {
    match read_to_string(&grep_args.path) {
        Ok(contents) => grep(contents, &grep_args),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    run(GrepArgs::from_args());
}
