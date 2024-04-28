use macroquad::prelude::*;
use ::rand::prelude::*;

mod  cell;
use cell::CellSqrt;

mod windows_lifegame;
use windows_lifegame::windows_param;

#[macroquad::main(windows_param)]
async fn main() {

    let frametime = std::time::SystemTime::now();
    let mut rng = thread_rng();
        
    let width = screen_width() as usize / 10;
    let height = screen_height() as usize / 10;
    
    // creation of all the cell 1000*1000 and size 10*10 (100 cells)
    let mut cells_list: Vec<CellSqrt> = (0..width * height)
        .map(|_| CellSqrt::new(rng.gen_bool(0.1)))
        .collect();
    
    loop{
        clear_background(BLACK);

        //create random alive cell
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                let index = (y as usize) * width + (x as usize);
                if cells_list[index].is_alive() {
                    draw_rectangle( (x * 10) as f32, (y * 10) as f32,10.0,10.0,WHITE,);
                }
            }
        }

        //find next status of each cell
        /* 
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                let index = (y as usize) * width + (x as usize);
                //faire itération des huits pixels autout de index
            }
        }*/

        // live time for the current frame
        draw_text(
            format!(
                "FrameTime: {}ms",frametime.elapsed().unwrap().as_secs_f32() * 1000.
            )
            .as_str(),
            5.,
            20.,
            20.,
            GREEN,
            );
        next_frame().await
    }   
}


    //draw_rectangle(0.0, 0.0, 10.0, 10.0, WHITE); //screen_width() / 2.0 - 60.0
    //créer rectangle en fonction 
    //set random cell alive


    /* 
    for y in 0..heigh_s as i32 {
        for x in 0..width_s as i32 {
            draw_rectangle(x_incr, 120.0, 5.0, 5.0, BLACK); //screen_width() / 2.0 - 60.0
        x_incr=x_incr+1.0;
        }
    }
    */

    //set_fullscreen(true);
    //draw_text("Conway's Game of Life in Rust", 20.0, 20.0, 30.0, DARKGRAY);