use crate::graphic::{MAP_HEIGHT, MAP_WIDTH};
use crate::helpers::{get_map_index_from_coordinates, get_x_for_angle, get_y_for_angle};
use crate::{CASE_SIZE, MAP_OFFSET_X, MAP_OFFSET_Y, MAP_SIZE};

use macroquad::prelude::KeyCode::*;
use macroquad::prelude::is_key_down;

pub const FOV: f32 = 60.0;
pub const PLAYER_SPAWN_X: f32 = 60.0 + CASE_SIZE as f32 / 2.0 as f32;
pub const PLAYER_SPAWN_Y: f32 = 60.0 + CASE_SIZE as f32 / 2.0 as f32;
pub const PLAYER_START_ANGLE: f32 = 0.0;
pub const PLAYER_MOVE_SPEED: f32 = 1.0;
pub const PLAYER_TURN_SPEED: f32 = 1.0;

pub struct Player{
    pub x: f32,
    pub y: f32,
    pub angle: f32 // Between 0° and 270 ° -> 0° is right
}

impl Player{
    pub fn get_player_drawn_x(&self)-> f32{
        return self.x + MAP_OFFSET_X as f32;
    }

    pub fn get_player_drawn_y(&self)-> f32{
        return self.y + MAP_OFFSET_Y as f32;
    }

    fn turn_right(&mut self){
        self.angle = (self.angle - PLAYER_TURN_SPEED) % 360.0;
    }

    fn turn_left(&mut self){
        self.angle = (self.angle + PLAYER_TURN_SPEED) % 360.0;
    }

    fn move_forward(&mut self, map : &[i32]){
        let end_x: f32 = get_x_for_angle(self.angle) * PLAYER_MOVE_SPEED;
        let end_y: f32 = get_y_for_angle(self.angle) * PLAYER_MOVE_SPEED;

        if self.can_move(map){
            self.x += end_x;
            self.y += end_y;
        }
    }

    fn move_backward(&mut self, map : &[i32]){
        let end_x: f32 = get_x_for_angle(self.angle) * PLAYER_MOVE_SPEED;
        let end_y: f32 = get_y_for_angle(self.angle) * PLAYER_MOVE_SPEED;

        if self.can_move(map){
            self.x -= end_x;
            self.y -= end_y;
        }
    }

    pub fn get_index_on_map_array(&self) -> (i32, i32){
        let map_x = self.x % MAP_OFFSET_X as f32 / CASE_SIZE as f32;
        let map_y = self.y % MAP_OFFSET_Y as f32 / CASE_SIZE as f32;

        return(f32::floor(map_x) as i32, f32::floor(map_y) as i32)
    }

    pub fn can_move(&self, map : &[i32]) -> bool{
        let (map_x, map_y) = self.get_index_on_map_array();
        let map_index = get_map_index_from_coordinates(map_y, map_x, MAP_WIDTH, MAP_HEIGHT) as usize;
        let mut can_move = true;

        if map[map_index] == 1{
            can_move = false;
        }

        return can_move;
    }

    pub fn handle_movements(&mut self, map : &[i32]){
        if is_key_down(W){
            self.move_forward(map);
        }else if is_key_down(S){
            self.move_backward(map);
        }
        
        if is_key_down(D){
            self.turn_right();

        }else if is_key_down(A) {
            self.turn_left();

        }
    }
}