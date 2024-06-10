use sdl2::{self, image::LoadTexture};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use statistics::{stats_layout, Stats};

mod cars;
use cars::handle_keyboard_event;

mod lane;
use lane::{Lane, Cross};

mod statistics;

pub fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;
    const VEHICULE_WIDTH: u32 = 25;
    const VEHICULE_HEIGHT: i32 = 50;
    const SAFETY_DISTANCE: i32 = 25;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("road intersection", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Load the background image
    let background_texture = texture_creator.load_texture("./assets/road.jpg").unwrap();

    // Load the font
    let font_path = "./assets/expressway.otf"; // Replace with the path to your font file
    let font = ttf_context.load_font(font_path, 128).unwrap();

    // Initialize lanes
    let mut lanes = vec![
        Lane::new(SAFETY_DISTANCE, Cross::First),
        Lane::new(SAFETY_DISTANCE, Cross::Second),
        Lane::new(SAFETY_DISTANCE, Cross::Third),
        Lane::new(SAFETY_DISTANCE, Cross::Fourth),
    ];
    let mut stats = Stats::new();


    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut id = 0;
    let mut i = 0;
    let mut is_stats = false;

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(55, 64, 5));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    if is_stats {
                        break 'running;
                    } else {
                        is_stats = true;
                    }
                }
                _ => {
                    id += 1;
                    handle_keyboard_event(&event, &mut lanes, WIDTH as i32, HEIGHT as i32, VEHICULE_WIDTH as i32, id);
                }
            }
        }

        if is_stats {
            stats_layout(&mut canvas, &mut stats, &font);
        } else {
            // Render the game
            canvas.copy(&background_texture, None, None).unwrap();

            let mut copy_lanes = lanes.clone();
            for lane in &mut lanes {
                lane.update(&mut stats, &mut canvas, WIDTH as i32, HEIGHT as i32, VEHICULE_WIDTH as i32, VEHICULE_HEIGHT, &mut copy_lanes);
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

