use ::std::env::args;
use ::std::fs;
use ::std::path::PathBuf;

use ::lalrpop_util::lalrpop_mod;

use crate::errors::build_error;

mod ast;
mod errors;

lalrpop_mod!(#[allow(clippy::all)] gen_parser, "/grammar.rs");

fn main() {
    let pth = PathBuf::from(args().nth(1).expect("provide input path as arg")).canonicalize().unwrap();
    let code = fs::read_to_string(&pth).unwrap();
    let parser = gen_parser::RootParser::new();
    match parser.parse(&code) {
        Ok(ast) => println!("ast:\n{}", serde_json::to_string_pretty(&ast).unwrap()),
        Err(err) => eprintln!("{}", build_error(err, pth.to_str().unwrap(), &code).0),
    }
}
