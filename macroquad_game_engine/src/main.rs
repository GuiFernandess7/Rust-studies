use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut x_position = 0.0;
    loop {
        clear_background(ORANGE);
        draw_rectangle(x_position, 100.0, 120.0, 60.0, BLACK);
        x_position += 10.0;
        if x_position > screen_width() {
            x_position = 0.0;
        }
        next_frame().await
    }
}