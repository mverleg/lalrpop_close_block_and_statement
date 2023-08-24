
mod ast;

use ::lalrpop_util::lalrpop_mod;

lalrpop_mod!(#[allow(clippy::all)] gen_parser, "/grammar.rs");

fn main() {
    println!("Hello, world!");
}
