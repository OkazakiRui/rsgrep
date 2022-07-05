use std::env::args;
use std::fs::read_to_string;

fn run_cat(path: String) {
    match read_to_string(path) {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    // match args().nth(1) {
    //     Some(path) => run_cat(path),
    //     None => println!("Usage: cat <file>"),
    // }

    if let Some(path) = args().nth(1) {
        run_cat(path);
    } else {
        println!("Usage: cat <file>");
    }
}
