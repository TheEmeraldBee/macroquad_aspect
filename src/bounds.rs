use macroquad::math::Vec2;

#[derive(Clone, Debug, Default)]
pub struct ScreenBounds {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
    pub aspect: f32,
}

impl ScreenBounds {
    pub fn new() -> Self {
        Self {
            top_left: Default::default(),
            bottom_right: Default::default(),
            aspect: 0.0,
        }
    }
}
