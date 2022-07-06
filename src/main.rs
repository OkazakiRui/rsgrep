use std::env::args;
use std::fs::read_to_string;

// fn run_cat(path: String) {
//     match read_to_string(path) {
//         Ok(contents) => println!("{}", contents),
//         Err(e) => println!("{}", e),
//     }
// }

fn grep(content: String, pattern: String) {
    for line in content.lines() {
        if line.contains(pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn run(path: String, pattern: String) {
    match read_to_string(path) {
        Ok(contents) => grep(contents, pattern),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    // match args().nth(1) {
    //     Some(path) => run_cat(path),
    //     None => println!("Usage: cat <file>"),
    // }

    let pattern = args().nth(1);
    let path = args().nth(1);

    match (pattern, path) {
        (Some(pattern), Some(path)) => run(path, pattern),
        _ => println!("Usage: grep <pattern> <file>"),
    }
}
