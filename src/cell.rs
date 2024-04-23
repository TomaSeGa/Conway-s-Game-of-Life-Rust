/* 

author : TSGD
This file defines the cells (pixels) that will be either alive or dead. 

*/

use macroquad::prelude::*;

pub struct cell {
    pos_x: f32, 
    pos_y: f32, 
    w: f32, 
    h: f32,
    pub color: Color,
    alive: bool,
}

impl cell {
    pub fn create(){
        draw_rectangle(screen_width() / 2.0 - 60.0, 120.0, 20.0, 20.0, WHITE);
    }
    pub fn remove(){

    }
    pub fn get_cell_status(){
        return self.alive; 
    }
}