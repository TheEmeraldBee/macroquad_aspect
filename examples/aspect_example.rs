extern crate macroquad;
extern crate macroquad_aspects;

use macroquad::prelude::*;
use macroquad_aspects::bounds::ScreenBounds;
use macroquad_aspects::prelude::*;


#[macroquad::main("Test")]
async fn main() {
    let aspects = vec![
        Aspect::new(1920.0, 1080.0),
        Aspect::new(1680.0, 1050.0)
    ];

    let mut render_target = render_target(aspects[0].width as u32, aspects[1].height as u32);
    let mut camera = Camera2D {
        render_target: Some(render_target),
        ..Default::default()
    };
    let mut screen_bounds = ScreenBounds::new();

    'running: loop {
        set_camera(&camera);
        clear_background(WHITE);

        draw_rectangle(0.0, 0.0, 50.0, 50.0, GREEN);

        draw_window(&mut camera, &mut render_target, &mut screen_bounds, aspects.clone());
        next_frame().await
    }
}