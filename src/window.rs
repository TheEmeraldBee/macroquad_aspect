use macroquad::camera::set_default_camera;
use macroquad::color::{BLACK, WHITE};
use macroquad::math::{Rect, Vec2, vec2};
use macroquad::prelude::{Camera2D, clear_background, draw_texture_ex, DrawTextureParams, render_target, RenderTarget, screen_height, screen_width};
use crate::aspect::{Aspect, Aspects};
use crate::bounds::ScreenBounds;

#[derive(Clone)]
pub struct WindowContext {
    pub camera: Camera2D,
    pub render_target: RenderTarget,
    pub screen_bounds: ScreenBounds,
    pub aspects: Aspects,
    /// If true will always take the closest aspect and scale the screen size based off that
    pub forced: bool,

    pub active_screen_size: Vec2,

    last_window_size: Vec2,
    cur_size: Vec2,
    active_aspect: Aspect
}

impl WindowContext {
    pub fn new(aspects: Aspects) -> Self {
        Self {
            camera: Default::default(),
            render_target: render_target(0, 0),
            screen_bounds: ScreenBounds {
                top_left: Default::default(),
                bottom_right: Default::default(),
                aspect: 0.0
            },
            aspects,
            forced: false,
            active_screen_size: vec2(0.0, 0.0),
            last_window_size: vec2(-100.0, -100.0),
            cur_size: Default::default(),
            active_aspect: Aspect::new(0.0, 0.0)
        }
    }
}

pub fn draw_window(context: &mut WindowContext) {

    let mut sizes = vec![];

    let mut dirty = false;

    if vec2(screen_width(), screen_height()) != context.last_window_size {
        dirty = true;

        if context.forced {
            for aspect in &context.aspects {
                let diff_width = (screen_width() - aspect.width as f32) * aspect.aspect;
                let diff_height = (screen_height() - aspect.height as f32) / aspect.aspect;

                let size;

                if diff_width <= diff_height {
                    size = vec2(screen_width(), screen_width() * aspect.aspect);
                } else {
                    size = vec2(screen_height() / aspect.aspect, screen_height())
                }

                sizes.push((aspect.clone(), size.clone()));
            }
        } else {
            let wanted_aspect = screen_height() / screen_width();
            let check_aspect = context.aspects[0].clone();

            let new_aspect = Aspect::new(check_aspect.height / wanted_aspect, check_aspect.height);

            sizes.push((new_aspect, vec2(screen_width(), screen_height())))
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


        context.cur_size = size;
        context.active_aspect = active_aspect.clone();
        context.active_screen_size = vec2(active_aspect.width.clone(), active_aspect.height.clone());
    }

    // Draw The Screen
    set_default_camera();

    clear_background(BLACK);

    draw_texture_ex(
        context.render_target.texture,
        screen_width() / 2.0 - context.cur_size.x / 2.0,
        screen_height() / 2.0 - context.cur_size.y / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(context.cur_size.clone()),
            flip_y: true,
            ..Default::default()
        },
    );

    // Only fix these issues if the screen needed to be updated
    if dirty {
        context.screen_bounds.top_left = vec2(screen_width() / 2.0 - context.cur_size.x / 2.0, screen_height() / 2.0 - context.cur_size.y / 2.0);
        context.screen_bounds.bottom_right = context.screen_bounds.top_left.clone() + context.cur_size;

        context.render_target = render_target(context.active_aspect.width as u32 * 12, context.active_aspect.height as u32 * 12);
        context.camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, context.active_aspect.width, context.active_aspect.height));
        context.camera.render_target = Some(context.render_target);
    }

    context.last_window_size = vec2(screen_width(), screen_height());
}