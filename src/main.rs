use std::{fs::File, io::BufWriter, path::Path, sync::Arc};

mod camera;
mod geometry;
mod material;
mod ray;
mod scene;
mod vec3;

use camera::Camera;
use geometry::Sphere;
use material::{LambertianMaterial, MetalMaterial};
use scene::Scene;
use vec3::*;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width: usize = 400;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 1000;
    let max_bounces = 30;

    // World
    let material_ground = LambertianMaterial::new(Color::new(0.8, 0.8, 0.0));
    let material_center = LambertianMaterial::new(Color::new(0.7, 0.3, 0.3));
    let material_left = MetalMaterial::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = MetalMaterial::new(Color::new(0.8, 0.6, 0.2), 1.0);

    let mut world: Scene = Scene::new();
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(material_ground),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Arc::new(material_center),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_left),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_right),
    )));

    // Camera
    let camera = Camera::new(aspect_ratio);

    // Render

    let mut pixels: Vec<u8> = vec![];

    for y in (0..height).rev() {
        for x in 0..width {
            let mut color_sampling = Color::default();

            for _ in 0..samples_per_pixel {
                let (u, v) = (
                    (x as f64 + rand::random::<f64>()) / (width as f64 - 1.0),
                    (y as f64 + rand::random::<f64>()) / (height as f64 - 1.0),
                );
                let ray = camera.ray_at(u, v);
                color_sampling += ray.color(&world, max_bounces);
            }

            let color_at_pixel: Color =
                (color_sampling / samples_per_pixel as f64).map(|v| v.sqrt());

            pixels.append(&mut color_at_pixel.rgb().to_vec());
        }
    }

    // Write

    let path = Path::new(r"./image.png");
    let file = File::create(path).expect("could not create image file");
    write_file(file, width, height, &pixels).expect("could not write image data");
}

fn write_file(
    file: File,
    width: usize,
    height: usize,
    pixels: &[u8],
) -> Result<(), png::EncodingError> {
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    let mut writer = encoder.write_header()?;

    writer.write_image_data(pixels)
}
