/* 

author : TSGD
This file defines the cells (pixels) that will be either alive or dead. 

*/

use macroquad::prelude::*;

pub struct CellSqrt {
    status: bool,
}

impl CellSqrt {
    pub fn new(alive: bool) -> Self {
        Self { status: alive }
    }

    pub fn is_alive(&self) -> bool {
        self.status
    }
}