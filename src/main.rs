mod player;
mod graphic;
mod helpers;

//External
use macroquad::prelude::*;

//Modules
use player::{Player, PLAYER_SPAWN_X, PLAYER_SPAWN_Y, PLAYER_START_ANGLE};
use graphic::{draw_map, draw_player, window_conf, MAP_OFFSET_X, MAP_OFFSET_Y};

const MAP_SIZE: usize = 49;

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player{x: PLAYER_SPAWN_X, y: PLAYER_SPAWN_Y, angle: PLAYER_START_ANGLE}; 

    //TODO: store the map in a file
    let map: [i32;  MAP_SIZE] = [
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
        
        player.handle_movements();

        draw_player(&player);

        next_frame().await;
    }

    //TODO: Make the raycaster
}