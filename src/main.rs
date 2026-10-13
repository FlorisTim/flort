mod compiler;
use macroquad::prelude::*;
use crate::compiler::CompilerConfig;

#[macroquad::main("Sketch")]
async fn main() {
    let a = compiler::get_tokens("func test(){hello;}", &CompilerConfig::default());
    for token in a {
        dbg!(token.typ);
        dbg!(token.text);
        println!("-----")
    }
    loop {
        clear_background(BLACK);

        draw_circle(400., 300., 50., RED);
        draw_line(0., 0., mouse_position().0, mouse_position().1, 2., WHITE);
        next_frame().await;
    }
}