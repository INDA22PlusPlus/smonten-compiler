use core::panic;
// use std::fs::File;

// use std::process::Command;
// use std::str::SplitAsciiWhitespace;
// use std::vec;

// mod read_file_emojis;
// use read_file_emojis::*;

mod tokenizer;
use tokenizer::Tokenizer;


mod parser;
use parser::Parser;

mod compiler;
use compiler::Compiler;

fn main() {

    let mut tokenizer = Tokenizer::new();
    // read_file_emojis::format_emojis();
    
    let tokens = match tokenizer.tokenize() {
        Err(e) => panic!("{}", e),
        Ok(token_vec) => token_vec,
    };
    // dbg!(&tokens);

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Err(e) => panic!("{}", e),
        Ok(ast_node) => ast_node,
    };
    // dbg!(&ast);

    let mut compiler = Compiler::new();
    match compiler.compile(ast) {
        Err(e) => panic!("{}", e),
        Ok(_) => (),
    }
    

}
