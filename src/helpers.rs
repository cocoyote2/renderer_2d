use std::{f32::consts::PI};

pub fn convert_degrees_to_radians(angle: f32) -> f32{
    return angle * PI / 180.0;
}

pub fn get_y_for_angle(angle: f32)-> f32{
    return f32::sin(convert_degrees_to_radians(angle));
}

pub fn get_x_for_angle(angle:f32) -> f32{
    return f32::cos(convert_degrees_to_radians(angle));
}