mod lexer;mod parser;mod common;
use std::fs;
use macroquad::prelude::*;
use crate::common::{CompilerConfig, ReadMode, Token, TokenType, Type};
use crate::lexer::Lexer;

#[macroquad::main("Sketch")]
async fn main(){
    let file = fs::read_to_string("source/test.java");
    let config = CompilerConfig::default();
    let mut lexer: Lexer = Lexer::new(&config);
    lexer.get_tokens(&*(file.unwrap_or_default().as_str().to_owned() + "\x04"));
    lexer.resolve_ambiguous();
    for token in &lexer.output_tokens {
        dbg!(&token.typ);
        dbg!(&token.text);
        println!("-----")
    }
    lexer.debug();


    loop {
        clear_background(BLACK);

        
        next_frame().await;
    }
}
