use ::std::env::args;
use ::std::fs;
use ::std::path::PathBuf;

use ::lalrpop_util::lalrpop_mod;

mod ast;

lalrpop_mod!(#[allow(clippy::all)] gen_parser, "/grammar.rs");

fn main() {
    let pth = PathBuf::from(args().nth(1).expect("provide input path as arg")).canonicalize().unwrap();
    eprintln!("code from {}", &pth.to_string_lossy()); //TODO @mark: TEMPORARY! REMOVE THIS!
    let code = fs::read_to_string(&pth).unwrap();
    eprintln!("code from {}: {}", &pth.to_string_lossy(), &code); //TODO @mark: TEMPORARY! REMOVE THIS!
    let parser = gen_parser::RootParser::new();
    dbg!(parser.parse(&code).unwrap());
}
