mod lexer;mod parser;mod common;
pub mod graphicslang;

use std::fs;
use macroquad::prelude::*;
use crate::common::{CompilerConfig};
use crate::graphicslang::{GraphicDrawer, GraphicParser};
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

    let mut gdrawer = GraphicDrawer::new();
    let mut gparser = GraphicParser::new();
    let file = fs::read_to_string("source/gui/draw/default.cve");
    gparser.parse_graphics_lang(&*file.unwrap_or_default());

    let std_atr = GraphicDrawer::attributes(
        200.0,
        75.0,
        100.0,
        100.0,
        Color::from_rgba(0,0,255, 255),
        Color::from_rgba(255,0,255, 255),
        25.0,
        0.0
    );
    println!("{:?}", gparser.output);
    set_default_font(load_ttf_font("source/gui/fonts/roboto.ttf").await.unwrap());
    loop {
        clear_background(BLACK);
        gdrawer.draw_element(gparser.output, std_atr);
        next_frame().await;

    }
}
