mod lexer;
mod parser;
mod common;

use macroquad::prelude::*;
use crate::common::CompilerConfig;

#[macroquad::main("Sketch")]
async fn main() {
    let a = lexer::get_tokens("if (a == b){cool();}", &CompilerConfig::default());
    for token in a {
        dbg!(token.typ);
        dbg!(token.text);
        dbg!(token.read);
        println!("-----")
    }
    loop {
        clear_background(BLACK);

        draw_circle(400., 300., 50., RED);
        draw_line(0., 0., mouse_position().0, mouse_position().1, 2., WHITE);
        next_frame().await;
    }
}