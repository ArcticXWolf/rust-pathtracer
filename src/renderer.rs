use rayon::prelude::*;

use crate::{camera::Camera, geometry::Hittable, vec3::Color};

pub fn render(
    world: &impl Hittable,
    camera: &Camera,
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    max_bounces: usize,
) -> Vec<u8> {
    (0..height)
        .into_par_iter()
        .rev()
        .flat_map(|y| {
            (0..width).into_par_iter().flat_map(move |x| {
                let mut color_sampling = Color::default();

                for _ in 0..samples_per_pixel {
                    let (u, v) = (
                        (x as f64 + rand::random::<f64>()) / (width as f64 - 1.0),
                        (y as f64 + rand::random::<f64>()) / (height as f64 - 1.0),
                    );
                    let ray = camera.ray_at(u, v);
                    color_sampling += ray.color(world, max_bounces);
                }

                let color_at_pixel: Color =
                    (color_sampling / samples_per_pixel as f64).map(|v| v.sqrt());

                color_at_pixel.rgb().to_vec()
            })
        })
        .collect()
}
