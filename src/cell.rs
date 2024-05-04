/* 

author : TSGD
This file defines the cells (pixels) that will be either alive or dead. 

*/

use macroquad::prelude::*;

#[derive(Clone)]
pub struct CellSqrt {
    pub status: bool,
}

impl CellSqrt {
    pub fn new(alive: bool) -> Self {
        Self { status: alive }
    }

    pub fn is_alive(&self) -> bool {
        self.status
    }

    pub fn count_live_neighbors(cells: &[CellSqrt], x: usize, y: usize, width: usize, height: usize) -> usize {
        let mut live_count = 0;
        let x = x as isize;
        let y = y as isize;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let nx = x + i;
                let ny = y + j;
                if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                    let idx = (ny * width as isize + nx) as usize;
                    if cells[idx].is_alive() {
                        live_count += 1;
                    }
                }
            }
        }
        live_count
    }
}