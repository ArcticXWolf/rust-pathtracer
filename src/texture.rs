use noise::{NoiseFn, Perlin};

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

pub struct PerlinNoiseTexture {
    noise: Box<Perlin>,
    scale: f64,
}

impl PerlinNoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Box::new(Perlin::new(rand::random())),
            scale,
        }
    }

    pub fn turbulance(&self, point: Vec3, depth: usize) -> f64 {
        let mut accumulator = 0.0;
        let mut p = point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accumulator += weight * self.noise.get(p.e);
            weight *= 0.5;
            p *= 2.0;
        }

        accumulator.abs()
    }
}

impl Texture for PerlinNoiseTexture {
    fn value(&self, _: f64, _: f64, point: Vec3) -> Color {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 - (self.scale * point.z() + 10.0 * self.turbulance(point, 7)).sin())
    }
}
