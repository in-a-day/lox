use lox::runner;
use std::env;

pub fn main() {
    let mut args = env::args();
    println!("{:?}", args);
    match args.len() {
        1 => runner::run_prompt(),
        2 => runner::run_file(&args.nth(1).unwrap()),
        _ => println!("Oops, something wrong."),
    }
}
