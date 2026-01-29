use std::{f32::consts::PI};

pub fn convert_degrees_to_radians(angle: f32) -> f32{
    return angle * PI / 180.0;
}

pub fn get_y_for_angle(angle: f32)-> f32{
    // return the opposite of sin because (0, 0) is top left on the screen
    return -f32::sin(convert_degrees_to_radians(angle));
}

pub fn get_x_for_angle(angle:f32) -> f32{
    return f32::cos(convert_degrees_to_radians(angle));
}

pub fn get_map_index_from_coordinates(y: i32, x: i32, col_number: i32, row_number: i32) -> i32{
    // MAP_OFFSET_X ET MAP_OFFSET_Y = 0, 0
    if y > row_number - 1 || x > col_number - 1{
        panic!("Coordinates out of bounds in function get_map_index_from_coordinates");
    }

    return y * col_number + x;
}

pub fn get_coordinates_from_map_index(index: i32, col_number: i32, row_number: i32) -> (i32, i32){
    if index > col_number * row_number - 1{
        panic!("Index out of bounds in function get_coordinates_from_map_index");
    }

    let x = index % col_number;
    let y = index / col_number;

    return (x, y);
}