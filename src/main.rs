
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas; 
use sdl2::video::Window; 
use std::time::Duration;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const ROAD_WIDTH: u32 = 120; 

fn draw_lane_markings(canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(255, 200, 0)); 

    let dash_length = 20;
    let gap_length = 15;
    let segment_length = dash_length + gap_length;

    let mid_y = (WINDOW_HEIGHT / 2) as i32;
    let intersection_start_x = (WINDOW_WIDTH / 2 - ROAD_WIDTH / 2) as i32;
    let intersection_end_x = (WINDOW_WIDTH / 2 + ROAD_WIDTH / 2) as i32;

    let mut current_x = 0;
    while current_x < intersection_start_x {
        let end_x = (current_x + dash_length).min(intersection_start_x);
        canvas.draw_line((current_x, mid_y), (end_x, mid_y))?;
        current_x += segment_length;
    }

    current_x = intersection_end_x;
    while current_x < WINDOW_WIDTH as i32 {
        let end_x = current_x + dash_length;
        canvas.draw_line((current_x, mid_y), (end_x, mid_y))?;
        current_x += segment_length;
    }


    let mid_x = (WINDOW_WIDTH / 2) as i32;
    let intersection_start_y = (WINDOW_HEIGHT / 2 - ROAD_WIDTH / 2) as i32;
    let intersection_end_y = (WINDOW_HEIGHT / 2 + ROAD_WIDTH / 2) as i32;

    let mut current_y = 0;
    while current_y < intersection_start_y {
        let end_y = (current_y + dash_length).min(intersection_start_y);
        canvas.draw_line((mid_x, current_y), (mid_x, end_y))?;
        current_y += segment_length;
    }

    current_y = intersection_end_y;
    while current_y < WINDOW_HEIGHT as i32 {
        let end_y = current_y + dash_length;
        canvas.draw_line((mid_x, current_y), (mid_x, end_y))?;
        current_y += segment_length;
    }

    Ok(())
}


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Road Intersection", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

       
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(100, 100, 100));
        let h_road = Rect::new(0, (WINDOW_HEIGHT / 2 - ROAD_WIDTH / 2) as i32, WINDOW_WIDTH, ROAD_WIDTH);
        let v_road = Rect::new((WINDOW_WIDTH / 2 - ROAD_WIDTH / 2) as i32, 0, ROAD_WIDTH, WINDOW_HEIGHT);
        canvas.fill_rect(h_road).map_err(|e| e.to_string())?;
        canvas.fill_rect(v_road).map_err(|e| e.to_string())?;

        draw_lane_markings(&mut canvas)?;

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
