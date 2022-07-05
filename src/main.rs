use std::fs::read_to_string;

fn run_cat() {
    let path = "./src/main.rs";
    match read_to_string(path) {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    run_cat();
}
