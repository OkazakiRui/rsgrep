use std::env::args;
use std::fs::read_to_string;

struct GrepArgs {
    pattern: String,
    path: String,
}
impl GrepArgs {
    fn new(pattern: String, path: String) -> GrepArgs {
        GrepArgs { pattern, path }
    }
    fn print_pattern(self, user: String) {
        let pat = self.pattern;
        println!("from: {}, pattern: {}", user, pat);
    }
}

fn grep(content: String, pattern: String) {
    for line in content.lines() {
        if line.contains(pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn run(grep_args: GrepArgs) {
    match read_to_string(grep_args.path) {
        Ok(contents) => grep(contents, grep_args.pattern),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    let path = args().nth(1);
    let pattern = args().nth(2);

    match (pattern, path) {
        (Some(pattern), Some(path)) => run(GrepArgs::new(pattern, path)),
        _ => println!("Usage: grep <pattern> <file>"),
    }

    GrepArgs::new("pattern".to_string(), String::from("path"))
        .print_pattern(String::from("r_okazaki"));
}
