use macroquad::prelude::*;

pub fn windows_param() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        window_width: 1000, 
        window_height: 1000, 
        window_resizable: true,
        ..Default::default()
    }
}