mod lexer;mod parser;mod common;
pub mod graphicslang;

use std::fs;
use macroquad::prelude::*;
use crate::common::{CompilerConfig, Token, TokenType};
use crate::graphicslang::{draw_element, GraphicParser};
use crate::lexer::Lexer;

#[macroquad::main("Sketch")]
async fn main(){
    let file = fs::read_to_string("source/code/test.java");
    let config = CompilerConfig::default();
    let mut lexer: Lexer = Lexer::new(&config);
    lexer.get_tokens(&*(file.unwrap_or_default() + "\x04"));
    for token in &lexer.output_tokens {
        dbg!(&token.typ);
        dbg!(&token.text);
        println!("-----")
    }
    let mut a = GraphicParser::new();
    a.parse_graphics_lang("f(test,hello,world,world,gorld,morld)");
    loop {
        clear_background(BLACK);
        draw_element(a.output);
        
        next_frame().await;
    }
}
