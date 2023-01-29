use std::{fs::File, io::BufWriter, ops::Neg, path::Path, sync::Arc};

mod bvh;
mod camera;
mod geometry;
mod material;
mod ray;
mod renderer;
mod vec3;

use bvh::BvhNode;
use camera::Camera;
use geometry::Hittable;
use geometry::Sphere;
use material::{DielectricMaterial, LambertianMaterial, Material, MetalMaterial};
use vec3::*;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width: usize = 1920;
    let samples_per_pixel = 500;
    let max_bounces = 50;

    // World
    let world = generate_scene();

    // Preview
    run(
        &world,
        aspect_ratio,
        width / 4,
        samples_per_pixel,
        max_bounces,
    );

    run(&world, aspect_ratio, width, samples_per_pixel, max_bounces);
}

fn run(
    world: &impl Hittable,
    aspect_ratio: f64,
    width: usize,
    samples_per_pixel: usize,
    max_bounces: usize,
) {
    let height = (width as f64 / aspect_ratio) as usize;

    // Camera
    let lookfrom = Vec3::new(12.0, 2.0, 4.0);
    let lookat = Vec3::new(0.0, 0.5, 0.0);
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

    let pixels: Vec<u8> = renderer::render(
        world,
        &camera,
        width,
        height,
        samples_per_pixel,
        max_bounces,
    );
    // Write

    let path = Path::new(r"./image.png");
    let file = File::create(path).expect("could not create image file");
    write_file(file, width, height, &pixels).expect("could not write image data");
}

fn generate_scene() -> BvhNode {
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let material_ground = Arc::new(LambertianMaterial::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            if -1 < b && b < 1 && -6 < a && a < 6 {
                continue;
            }

            let center = Vec3::new(
                a as f64 + 0.5 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            let radius = 0.2;

            let (material, is_glass): (Arc<dyn Material>, bool) = match rand::random::<f64>() {
                x if x < 0.6 => {
                    let albedo = Color::random() * Color::random();
                    (Arc::new(LambertianMaterial::new(albedo)), false)
                }
                x if x < 0.8 => {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand::random::<f64>();
                    (Arc::new(MetalMaterial::new(albedo, fuzz)), false)
                }
                _ => (Arc::new(DielectricMaterial::new(1.5)), true),
            };

            if is_glass && rand::random::<f64>() < 0.5 {
                world.push(Box::new(Sphere::new(center, radius, material.clone())));
                world.push(Box::new(Sphere::new(center, radius.neg() + 0.02, material)));
            } else {
                world.push(Box::new(Sphere::new(center, radius, material)));
            }
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

    BvhNode::new(world)
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
