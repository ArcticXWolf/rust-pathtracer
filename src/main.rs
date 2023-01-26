use std::{fs::File, io::BufWriter, path::Path, sync::Arc};

mod camera;
mod geometry;
mod material;
mod ray;
mod scene;
mod vec3;

use camera::Camera;
use geometry::Sphere;
use material::{DielectricMaterial, LambertianMaterial, Material, MetalMaterial};
use scene::Scene;
use vec3::*;

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let width: usize = 1200;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_bounces = 50;

    // World
    let world = generate_scene();

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        up,
        20.0,
        aspect_ratio,
        aperture,
        focus_dist,
    );

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
        println!("Finished line {}", y);
    }

    // Write

    let path = Path::new(r"./image.png");
    let file = File::create(path).expect("could not create image file");
    write_file(file, width, height, &pixels).expect("could not write image data");
}

fn generate_scene() -> Scene {
    let mut world: Scene = Scene::new();

    let material_ground = Arc::new(LambertianMaterial::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            let radius = 0.2;

            let material: Arc<dyn Material> = match rand::random::<f64>() {
                x if x < 0.8 => {
                    let albedo = Color::random() * Color::random();
                    Arc::new(LambertianMaterial::new(albedo))
                }
                x if x < 0.95 => {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand::random::<f64>();
                    Arc::new(MetalMaterial::new(albedo, fuzz))
                }
                _ => Arc::new(DielectricMaterial::new(1.5)),
            };

            world.push(Box::new(Sphere::new(center, radius, material)));
        }
    }

    let material_big1 = Arc::new(DielectricMaterial::new(1.5));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_big1,
    )));

    let material_big2 = Arc::new(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material_big2,
    )));

    let material_big3 = Arc::new(MetalMaterial::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material_big3,
    )));

    world
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
