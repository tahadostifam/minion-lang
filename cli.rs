use std::{env, fs, ops::Index};
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("No such file or directory.");
        std::process::exit(1);
    }

    let file_path = args.index(1);

    match fs::read_to_string(file_path) {
        Err(err) => {
            println!("(taha): {}", err);
        }
        Ok(file_content) => {
            let node = Parser::parse(file_content).unwrap();

            dbg!(node);
        }
    }
}
