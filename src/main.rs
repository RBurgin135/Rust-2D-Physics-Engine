use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_circle(screen_width() /2.0, screen_height() /2.0, 15.0, YELLOW);

        //define floor
        let thickness = 3.0;
        let vertical_buffer = 50.0;
        draw_line(0.0, screen_height()-vertical_buffer, screen_width(), screen_height()-vertical_buffer, thickness, BLUE);

        next_frame().await
    }
}