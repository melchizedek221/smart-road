use sdl2::{self, image::LoadTexture};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
// use sdl2::rect::Rect;
use std::time::Duration;

// mod map;
// use map::draw_map;


mod cars;
use cars::handle_keyboard_event;

mod lane;
use lane::{Lane, Cross};

pub fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;
    const VEHICULE_WIDTH: u32 = 25;
    const VEHICULE_HEIGHT: i32 = 50;
    const SAFETY_DISTANCE: i32 = 25;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("road intersection", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();

    let mut lanes = vec![
        Lane::new(SAFETY_DISTANCE, Cross::First),
        Lane::new(SAFETY_DISTANCE, Cross::Second),
        Lane::new(SAFETY_DISTANCE, Cross::Third),
        Lane::new(SAFETY_DISTANCE, Cross::Fourth),
    ];

    let mut id = 0;

    //adding bg img instead of color
    let texture_creator = canvas.texture_creator();
    let background_texture = texture_creator.load_texture("./assets/road.jpg").unwrap();
    
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(55, 64, 5));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {
                    id += 1;
                    handle_keyboard_event(&event, &mut lanes, WIDTH as i32, HEIGHT as i32, VEHICULE_WIDTH as i32, id);
                }
            }
        }

        canvas.clear();
        // The rest of the game loop goes here...
        
        // Copy the texture to the canvas
        canvas.copy(&background_texture, None, None).unwrap();

        // map
        // draw_map(&mut canvas, WIDTH as i32, HEIGHT as i32, VEHICULE_WIDTH as i32);
        let copy_lanes =&mut lanes.clone();
        for lane in &mut lanes {
            lane.update(&mut canvas, WIDTH as i32, HEIGHT as i32, VEHICULE_WIDTH as i32, VEHICULE_HEIGHT,copy_lanes);
        };

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
