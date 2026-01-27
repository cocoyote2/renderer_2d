use crate::{MAP_OFFSET_X, MAP_OFFSET_Y};

pub const FOV: f32 = 60.0;

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
}