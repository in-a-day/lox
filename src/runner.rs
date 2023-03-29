use std::{
    fs::File,
    io::{stdin, stdout, Write},
};

use crate::{
    exercise::ch5::AstVisitor,
    parser::Parser,
    scanner::Scanner,
    token::{Token, TokenType},
};
static mut HAS_ERR: bool = false;

pub fn error(line: u32, message: &str) {
    report(line, "", message);
}

pub fn error_token(token: &Token, message: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), message)
    }
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
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();
    if !unsafe { HAS_ERR } && expr.is_some() {
        println!("{}", expr.unwrap().visit(&AstVisitor));
    }
}

#[test]
fn run_it() {
    run_prompt();
}
