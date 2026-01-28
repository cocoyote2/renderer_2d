use macroquad::prelude::*;

use crate::player::{Player, FOV};
use crate::helpers::{convert_degrees_to_radians, get_y_for_angle, get_x_for_angle};

pub const CASE_SIZE: i32 = 64;
pub const MAP_WIDTH: i32 = 7;
pub const MAP_HEIGHT: i32 = 7;
pub const WINDOW_WIDTH: i32 = 1920;
pub const WINDOW_HEIGHT: i32 = 1080;
pub const GRID_WIDTH: i32 = MAP_WIDTH * CASE_SIZE;
pub const GRID_HEIGHT: i32 = MAP_HEIGHT * CASE_SIZE;
pub const MAP_OFFSET_X: i32 = (WINDOW_WIDTH - GRID_WIDTH) / 2;
pub const MAP_OFFSET_Y: i32 = (WINDOW_HEIGHT - GRID_HEIGHT) / 2;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Raycaster".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn draw_map(map: [i32; 49], map_offset_x: i32, map_offset_y: i32){
    for (i, elem) in map.iter().enumerate() {
        let curr_col = (i as i32) % MAP_WIDTH;
        let curr_row = (i as i32) / MAP_WIDTH;

        if *elem == 1 {
            draw_rectangle(
                (curr_col * CASE_SIZE + map_offset_x) as f32, 
                (curr_row * CASE_SIZE + map_offset_y) as f32, 
                CASE_SIZE as f32, 
                CASE_SIZE as f32, 
                RED
            );
        }else if *elem == 0 {
            draw_rectangle(
                (curr_col * CASE_SIZE + map_offset_x) as f32,
                (curr_row * CASE_SIZE + map_offset_y) as f32, 
                CASE_SIZE as f32, 
                CASE_SIZE as f32, 
                BLACK
            );
        }
    }
}

pub fn draw_player(player: &Player){
    draw_circle(player.get_player_drawn_x(), player.get_player_drawn_y(), CASE_SIZE as f32 / 2.0, GREEN);
    draw_field_vision(player);
}

fn draw_field_vision(player: &Player){
    // Directions
    let x_for_angle = get_x_for_angle(player.angle);
    let y_for_angle = get_y_for_angle(player.angle);

    let end_x = player.x + MAP_OFFSET_X as f32 + x_for_angle * 200.0;
    let end_y = player.y + MAP_OFFSET_Y as f32 + y_for_angle * 200.0;

    draw_line(
        player.x + MAP_OFFSET_X as f32, 
        player.y + MAP_OFFSET_Y as f32, 
        end_x,
        end_y,
        5.0, 
        GREEN
    );

    //TODO: draw the second line
}