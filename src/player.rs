use crate::helpers::{get_x_for_angle, get_y_for_angle};
use crate::{MAP_OFFSET_X, MAP_OFFSET_Y, CASE_SIZE};

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

    fn move_forward(&mut self){
        //TODO: check for collisions
        let end_x: f32 = get_x_for_angle(self.angle) * PLAYER_MOVE_SPEED;
        let end_y: f32 = get_y_for_angle(self.angle) * PLAYER_MOVE_SPEED;
            
        self.x += end_x;
        self.y += end_y;
    }

    fn move_backward(&mut self){
        //TODO: check for collisions
        let end_x: f32 = get_x_for_angle(self.angle);
        let end_y: f32 = get_y_for_angle(self.angle);

        self.x -= end_x * PLAYER_MOVE_SPEED;
        self.y -= end_y * PLAYER_MOVE_SPEED;
    }

    fn check_for_collisions(self){
        // I need to know the next place where the player is gonna move to
        // and avoid it or allow it
    }

    pub fn handle_movements(&mut self){
        if is_key_down(W){
            self.move_forward();

        }else if is_key_down(S){
            self.move_backward();

        }
        
        if is_key_down(D){
            self.turn_right();

        }else if is_key_down(A) {
            self.turn_left();

        }
    }
}