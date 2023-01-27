use std::time::Instant;

use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::{camera::Camera, scene::Scene, vec3::Color};

pub fn render(
    world: &Scene,
    camera: &Camera,
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    max_bounces: usize,
) -> Vec<u8> {
    const PROGRESS_BAR_INCREMENT: usize = 64;
    let bar = &Box::new(ProgressBar::new(
        (width * height / PROGRESS_BAR_INCREMENT) as u64,
    ));
    bar.set_prefix("💻 Rendering");
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:.white} [{elapsed_precise}/{duration_precise}] {bar:40.green/green} {percent}%")
            .expect("template error for indicatif"),
    );

    let timer_start = Instant::now();

    let pixels = (0..height)
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

                if x % PROGRESS_BAR_INCREMENT == 0 {
                    bar.inc(1);
                }

                color_at_pixel.rgb().to_vec()
            })
        })
        .collect();

    let timer_end = timer_start.elapsed();

    bar.finish();

    println!("Finished ({})", HumanDuration(timer_end));

    pixels
}
