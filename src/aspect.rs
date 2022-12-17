
pub type Aspects = Vec<Aspect>;

#[derive(Clone)]
pub struct Aspect {
    pub width: f32,
    pub height: f32,
    pub aspect: f32
}

impl Aspect {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            aspect: height / width
        }
    }
}