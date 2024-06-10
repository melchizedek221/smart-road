use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Font;

// use crate::lane:: Lane;
#[derive(Clone, Debug)]
pub struct Stats {
    pub vehicpass: u32,
    pub count_vec_spawn: i32, 
    pub max_speed: f64,
    pub min_speed: f64,
    pub average_speed: f32,
    pub max_time: f64,
    pub min_time: f64,
    pub close_calls: u32,
    pub durations: Vec<f64>,
    pub velocities: Vec<f64>,

}

impl Stats {
    pub fn new() -> Self {
        Stats {
            vehicpass: 0,
            count_vec_spawn: 0, 
            max_speed: 0.0,
            min_speed: 0.0,
            average_speed: 0.0,
            max_time: 0.0,
            min_time: 0.0,
            close_calls: 0,
            durations: Vec::new(),
            velocities: Vec::new(),
        }
    }

    pub fn update_stats(&mut self) {
        // println!("all vecs : {:?}", self.durations);

        self.max_time = find_max(&self.durations);
        self.max_speed = find_max(&self.velocities);
        
        self.min_time = find_min(&self.durations);
        self.min_speed = find_min(&self.velocities);
    }
}

fn find_max(vec: &Vec<f64>) -> f64 {
    vec.iter()
        .cloned()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0)
}

fn find_min(vec: &Vec<f64>) -> f64 {
    vec.iter()
        .cloned()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0)
}

pub fn stats_layout(canvas: &mut WindowCanvas, stats: &mut Stats, font: &Font) {
    canvas.clear();
    let mut surface = font
        .render("STATISTICS")
        .blended(sdl2::pixels::Color::BLACK)
        .unwrap();
    let mut size = surface.size();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let mut rect = Rect::new(280, 280, size.0 / 8, size.1 / 8);
    canvas.copy(&texture, None, rect).unwrap();

    let mut _text = String::new(); 

    for i in 0..6 {
        match i {
            0 => _text = format!("Passed vehicules number: {}", stats.vehicpass),
            1 => _text = format!("Max speed: {} pxs/s", stats.max_speed),
            2 => _text = format!("Min speed: {} psx/s", stats.min_speed),
            3 => _text = format!("Max time: {:.2} s", stats.max_time),
            4 => _text = format!("Min time: {:.2} s", stats.min_time), 
            _ => _text = format!("Collisions: {}", 0),
        }
    surface = font
            .render(&_text)
            .blended(sdl2::pixels::Color::BLACK)
            .unwrap();
        size = surface.size();
        rect = Rect::new(280, 320 + 25 * i, size.0 / 8, size.1 / 8);
        texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        canvas.copy(&texture, None, rect).unwrap();
    }
}
