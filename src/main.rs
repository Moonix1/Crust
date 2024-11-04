use std::{env::args, fs::File, io::Read};

mod token;
mod lexer;
mod parser;

use lexer::*;
use parser::*;

fn get_file_contents(path: &str) -> String {
	let mut file = File::open(path).unwrap();
	let mut buf = String::new();

	let _ = file.read_to_string(&mut buf);

	buf
}

fn main() {
	let args: Vec<String> = args().collect();

	let contents = get_file_contents(&args[1]);
	let mut lexer = Lexer::init(contents);

	let mut parser = Parser::init(lexer);
	let func = parser.parse_func();

	println!("{:?}", func);
}
