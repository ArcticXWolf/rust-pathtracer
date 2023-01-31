use std::{fs::File, io::BufWriter, path::Path};

mod bvh;
mod camera;
mod geometry;
mod material;
mod ray;
mod renderer;
mod scene;
mod texture;
mod vec3;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use scene::Scene;
use scene::*;

fn main() {
    let scene: Box<dyn Scene> = Box::new(CornellBoxScene {});
    let world = scene.get_world();
    let settings = scene.get_output_settings();
    let amount_of_frames = match settings {
        scene::OutputSettings::StaticImage { image_settings: _ } => 1,
        scene::OutputSettings::Animation {
            image_settings: _,
            ref fps,
            ref duration,
        } => (fps * duration) as u64,
    };
    let image_settings = match settings {
        scene::OutputSettings::StaticImage { ref image_settings } => image_settings,
        scene::OutputSettings::Animation {
            ref image_settings,
            fps: _,
            duration: _,
        } => image_settings,
    };

    let bar_style = ProgressStyle::default_bar()
            .template("{prefix:.white} [{elapsed_precise}/{duration_precise}] {bar:40.green/green} {percent}%")
            .expect("template error for indicatif");

    let frame_progress = ProgressBar::new(amount_of_frames);
    frame_progress.set_prefix("💻 Rendering Frames ");
    frame_progress.set_style(bar_style);
    frame_progress.tick();

    for frame_index in 0..(amount_of_frames as usize) {
        let t = (frame_index as f64) / amount_of_frames as f64;
        let camera = scene.get_camera_at(t);

        // Render
        let pixels: Vec<u8> = renderer::render(
            &world,
            &camera,
            image_settings.width,
            image_settings.height,
            image_settings.background,
            image_settings.samples_per_pixel,
            image_settings.max_bounces,
        );

        // Write PNG
        let path_str = format!("./output/image_{:04}.png", frame_index);
        let path = Path::new(&path_str);
        let file = File::create(path).expect("could not create image file");
        write_file(file, image_settings.width, image_settings.height, &pixels)
            .expect("could not write image data");

        frame_progress.inc(1);
    }
    frame_progress.finish();
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
