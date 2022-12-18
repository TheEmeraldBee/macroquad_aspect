extern crate macroquad;
extern crate macroquad_aspect;

use macroquad::prelude::*;
use macroquad_aspect::prelude::*;


#[macroquad::main("Test")]
async fn main() {
    let mut window_context = WindowContext::new(
        vec![
        Aspect::new(1920.0, 1080.0),
        Aspect::new(1680.0, 1050.0)]
    );

    loop {
        set_camera(&window_context.camera);
        clear_background(WHITE);

        draw_rectangle(0.0, 0.0, 50.0, 50.0, GREEN);

        draw_window(&mut window_context);
        next_frame().await
    }
}