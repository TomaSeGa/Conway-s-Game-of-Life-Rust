use macroquad::prelude::*;

#[macroquad::main("Game of Life")]
async fn main() {
    loop {
        clear_background(BLACK);

        //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 120.0, 20.0, 20.0, WHITE);
        //draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("Conway's Game of Life in Rust", 20.0, 20.0, 30.0, DARKGRAY);
        
        next_frame().await
    }
}

