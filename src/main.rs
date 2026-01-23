use macroquad::prelude::*;


const CASE_SIZE: i32 = 64;
const MAP_WIDTH: i32 = 7;
const MAP_HEIGHT: i32 = 7;
const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 1080;

fn window_conf() -> Conf {
    Conf {
        window_title: "Raycaster".to_owned(),
        window_width: 1920,
        window_height: 1080,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn draw_map(map: [i32; 49], map_offset_x: i32, map_offset_y: i32){
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

#[macroquad::main(window_conf)]
async fn main() {
    let grid_width = MAP_WIDTH * CASE_SIZE;
    let grid_height = MAP_HEIGHT * CASE_SIZE;
    let map_offset_x = (WINDOW_WIDTH - grid_width) / 2;
    let map_offset_y = (WINDOW_HEIGHT - grid_height) / 2;

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

        draw_map(map, map_offset_x, map_offset_y);

        next_frame().await
    }

    //TODO: Make the raycaster
}