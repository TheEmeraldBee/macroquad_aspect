use macroquad::camera::set_default_camera;
use macroquad::color::{BLACK, WHITE};
use macroquad::math::{Vec2, vec2};
use macroquad::prelude::{Camera2D, clear_background, draw_texture_ex, DrawTextureParams, render_target, RenderTarget, screen_height, screen_width};
use crate::aspect::{Aspect, Aspects};
use crate::bounds::ScreenBounds;

pub fn draw_window(camera: &mut Camera2D, render_target: &mut RenderTarget, screen_bounds: &mut ScreenBounds, aspects: Aspects) {

    let mut sizes = vec![];

    for aspect in &aspects {
        let diff_width = (screen_width() - aspect.width as f32) * aspect.aspect;
        let diff_height = (screen_height() - aspect.height as f32) / aspect.aspect;

        #[allow(unused_assignments)]
        let mut size = Vec2::default();

        if diff_width <= diff_height {
            size = vec2(screen_width(), screen_width() * aspect.aspect);
        } else {
            size = vec2(screen_height() / aspect.aspect, screen_height())
        }

        sizes.push((aspect.clone(), size.clone()));
    }

    let mut size = vec2(0.0, 0.0);
    let mut active_aspect = Aspect::new(0.0, 0.0);

    for (aspect, check_size) in &sizes {
        if check_size.x > size.x || check_size.y > size.y {
            size = check_size.clone();
            active_aspect = aspect.clone();
        }
    }

    if size.x == 0.0 || size.y == 0.0 {
        panic!("draw_window(): Size included Zero")
    }

    // Draw The Screen
    set_default_camera();

    clear_background(BLACK);

    draw_texture_ex(
        render_target.texture,
        screen_width() / 2.0 - size.x / 2.0,
        screen_height() / 2.0 - size.y / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(size.clone()),
            flip_y: true,
            ..Default::default()
        },
    );

    screen_bounds.top_left = vec2(screen_width() / 2.0 - size.x / 2.0, screen_height() / 2.0 - size.y / 2.0);
    screen_bounds.bottom_right = screen_bounds.top_left.clone() + size;

    *render_target = macroquad::prelude::render_target(active_aspect.width as u32, active_aspect.height as u32);
    camera.zoom = vec2(1.0 / active_aspect.width, 1.0 / active_aspect.height);
    camera.render_target = Some(*render_target);
}