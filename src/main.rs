mod compiler;

use macroquad::prelude::*;

#[macroquad::main("Sketch")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_circle(400., 300., 50., RED);
        draw_line(0., 0., mouse_position().0, mouse_position().1, 2., WHITE);
        next_frame().await;
    }
}