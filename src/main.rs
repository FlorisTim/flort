mod lexer;
mod parser;
mod common;

use std::fs;
use std::env;
use std::path::Path;
use macroquad::prelude::*;
use crate::common::CompilerConfig;

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
    loop {
        clear_background(BLACK);

        
        next_frame().await;
    }
}