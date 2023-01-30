use crate::vec3::{Color, Vec3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Color;
}

pub struct SolidColorTexture {
    color: Color,
}

impl SolidColorTexture {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _: f64, _: f64, _: Vec3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>) -> Self {
        Self { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Color {
        let sines = (10.0 * point.x()).sin() * (10.0 * point.y()).sin() * (10.0 * point.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}
