use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Font;

use crate::Stats;


pub fn stats_layout(canvas: &mut WindowCanvas, stats: Stats, font: &Font) {
    canvas.clear();
    let mut surface = font
        .render("statistics")
        .blended(sdl2::pixels::Color::BLACK)
        .unwrap();
    let mut size = surface.size();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let mut rect = Rect::new(280, 280, size.0 / 8, size.1 / 8);
    canvas.copy(&texture, None, rect).unwrap();
    for i in 0..7 {
        let mut text = format!("min time: {} s", stats.min_time);
        match i {
            0 => text = format!("avg velocity: {:.2} pxs/s", stats.average_velocity),
            1 => text = format!("total cars: {}", stats.total_cars),
            2 => text = format!("nbr of passed cars: {}", stats.nbr_passed),
            3 => text = format!("max velocity: {} pxs/s", stats.max_velocity),
            4 => text = format!("min velocity: {} psx/s", stats.min_velocity),
            5 => text = format!("max time: {} s", stats.max_time),
            _ => {}
        }
        surface = font
            .render(&text)
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
