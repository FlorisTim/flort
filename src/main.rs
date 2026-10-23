mod lexer;
mod parser;
mod common;

use std::fs;
use macroquad::prelude::*;
use crate::common::{CompilerConfig, ReadMode, Token, TokenType, Type};
use crate::parser::parse_value;

#[macroquad::main("Sketch")]
async fn main() {
    let file = fs::read_to_string("source/test.java");
    let a = lexer::resolve_ambiguous(lexer::get_tokens(file.unwrap().as_str(), &CompilerConfig::default()),&CompilerConfig::default());
    for token in a {
        dbg!(token.typ);
        dbg!(token.text);
        dbg!(token.read);
        println!("-----")
    }

    println!("float parse test");
    println!("{:?}", parse_value(Token{text: "5f".to_string(), typ: TokenType::Operand, read: ReadMode::Normal}, &CompilerConfig::default()));

    loop {
        clear_background(BLACK);

        
        next_frame().await;
    }
}
