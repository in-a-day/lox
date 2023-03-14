use std::{
    fs::File,
    io::{stdin, stdout, Write},
};

use crate::scanner::Scanner;
static mut HAS_ERR: bool = false;

pub fn error(line: u32, message: &str) {
    report(line, "", message);
}

fn report(line: u32, position: &str, message: &str) {
    unsafe {
        HAS_ERR = true;
    }
    println!("[line{line}] Error{position}: {message}");
}

pub fn run_file(path: &str) {
    let cnt = std::io::read_to_string(File::open(path).unwrap()).unwrap();
    run(&cnt);
}

#[allow(unused_must_use)]
pub fn run_prompt() {
    let mut cnt = String::new();
    loop {
        cnt.clear();
        print!("> ");
        stdout().flush();
        if stdin().read_line(&mut cnt).is_err() {
            break;
        }
        run(&cnt);
    }
}

fn run(source: &str) {
    let scanner = Scanner::new(source.to_owned());
    let tokens = scanner.scan();
    for token in tokens {
        println!("{:?}", token);
    }
}
