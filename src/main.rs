mod player;
mod graphic;
mod helpers;

//External
use macroquad::prelude::*;

//Modules
use player::Player;
use graphic::{draw_map, draw_player, window_conf, MAP_OFFSET_X, MAP_OFFSET_Y};

#[macroquad::main(window_conf)]
async fn main() {
    let player = Player{x: 60.0, y: 60.0, angle: 0.0}; 

    //TODO: store the map in a file
    let map: [i32;  49] = [
        1, 1, 1, 1, 1, 1, 1,
        1, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 1, 1, 0, 1,
        1, 0, 0, 0, 1, 0, 1,
        1, 0, 0, 1, 1, 0, 1,
        1, 0, 0, 0, 0, 0, 1,
        1, 1, 1, 1, 1, 1, 1  
    ];

    loop{
        clear_background(LIGHTGRAY);

        draw_map(map, MAP_OFFSET_X, MAP_OFFSET_Y);

        draw_player(&player);

        next_frame().await;
    }

    //TODO: Make the raycaster
}