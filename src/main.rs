use macroquad::prelude::*;
use ::rand::prelude::*;
use std::{thread, time};

mod cell;
use cell::CellSqrt;

mod windows_lifegame;
use windows_lifegame::windows_param;

const TIME_TO_SLEEP_BETWEEN_CYCLES: u64 = 100;

#[macroquad::main(windows_param)]
async fn main() {
    let frametime = std::time::SystemTime::now();
    let mut rng = thread_rng();

    let width = screen_width() as usize / 10;
    let height = screen_height() as usize / 10;

    /* Create random grid */
    let mut cells_list: Vec<CellSqrt> = (0..width * height)
        .map(|_| CellSqrt::new(rng.gen_bool(0.5)))
        .collect();

    /* Create random grid */

    loop {
        clear_background(BLACK);

        // can create your own pixel
        /* 
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            draw_rectangle(mouse_x, mouse_y, 10.0, 10.0, WHITE);
        }*/

        for y in 0..height as i32 {
            for x in 0..width as i32 {
                let index = (y as usize) * width + (x as usize);
                if cells_list[index].is_alive() {
                    draw_rectangle((x * 10) as f32, (y * 10) as f32, 10.0, 10.0, BLUE);
                }
            }
        }

        let mut next_gen: Vec<CellSqrt> = cells_list.clone();

        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                let live_neighbors = CellSqrt::count_live_neighbors(&cells_list, x, y, width, height);
                /* Une cellule vivante possédant deux ou trois cellules voisines vivantes le reste, sinon elle meurt. */
                if cells_list[index].is_alive() 
                {
                    //logic for live cells
                    if (live_neighbors >= 2) && (live_neighbors < 3)
                    {
                        next_gen[index].status = true;
                    } 
                    else 
                    {
                        next_gen[index].status = false;
                    }
                    
                } 
                /* Une cellule morte possédant exactement trois cellules voisines vivantes devient vivante (elle naît)  */
                else 
                {
                    // logic for dead cells
                    if live_neighbors == 3
                    {
                        next_gen[index].status = true;
                    } 
                    else 
                    {
                        next_gen[index].status = false;
                    }
                }
            }
        }

        cells_list = next_gen;


        draw_text(
            format!("FrameTime: {}ms", frametime.elapsed().unwrap().as_secs_f32() * 1000.)
                .as_str(),
            5.,
            20.,
            20.,
            GREEN,
        );
        
        thread::sleep(time::Duration::from_millis(TIME_TO_SLEEP_BETWEEN_CYCLES));
        /*  
        let minimum_frame_time = 1. / 60.; // 60 FPS
        let frame_time = get_frame_time();
        //println!("Frame time: {}ms", frame_time * 1000.);
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            //println!("Sleep for {}ms", time_to_sleep);
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }*/

        next_frame().await
    }
}
