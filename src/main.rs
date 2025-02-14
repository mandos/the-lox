mod lox;
mod scanner;

use lox::Lox;
use std::env::args;
use std::process::exit;

fn main() {
    let args: Vec<String> = args().collect();
    println!("args: {:?}", args);

    let mut lox = Lox::new();

    if args.len() > 2 {
        println!("Usage: the-lox [script]");
        exit(64);
    } else if args.len() == 2 {
        lox.run_file(args.get(1).expect("Doesn't have 1 parameter"));
    } else {
        lox.run_prompt();
    }
}
