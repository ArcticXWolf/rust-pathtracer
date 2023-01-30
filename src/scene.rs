use std::{ops::Neg, sync::Arc};

use crate::{
    bvh::BvhNode,
    camera::Camera,
    geometry::{Hittable, Sphere},
    material::{DielectricMaterial, LambertianMaterial, Material, MetalMaterial},
    texture::{CheckerTexture, SolidColorTexture},
    vec3::{Color, Vec3},
};

pub struct ImageSettings {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub max_bounces: usize,
}

pub enum OutputSettings {
    StaticImage {
        image_settings: ImageSettings,
    },
    Animation {
        image_settings: ImageSettings,
        fps: f64,
        duration: f64,
    },
}

pub trait Scene {
    fn get_world(&self) -> BvhNode;
    fn get_camera_at(&self, t: f64) -> Camera;
    fn get_output_settings(&self) -> OutputSettings;
}

pub struct SphereFieldScene;

impl Scene for SphereFieldScene {
    fn get_output_settings(&self) -> OutputSettings {
        OutputSettings::Animation {
            image_settings: ImageSettings {
                width: 854,
                height: 480,
                samples_per_pixel: 250,
                max_bounces: 20,
            },
            fps: 30.0,
            duration: 10.0,
        }
    }

    fn get_camera_at(&self, t: f64) -> Camera {
        let lookfrom = Vec3::new(
            12.0 * (2.0 * std::f64::consts::PI * t).cos(),
            1.0 + 2.0 * (std::f64::consts::PI * t).sin(),
            12.0 * (2.0 * std::f64::consts::PI * t).sin(),
        );
        let lookat = Vec3::new(0.0, 0.5, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let focus_dist = 10.0;
        let aperture = 0.1;
        let aspect_ratio = match self.get_output_settings() {
            OutputSettings::Animation {
                image_settings,
                fps: _,
                duration: _,
            } => image_settings.width as f64 / image_settings.height as f64,
            _ => unimplemented!(),
        };

        Camera::new(
            lookfrom,
            lookat,
            up,
            20.0,
            aspect_ratio,
            aperture,
            focus_dist,
        )
    }

    fn get_world(&self) -> BvhNode {
        let mut world: Vec<Box<dyn Hittable>> = vec![];

        let checker_texture = CheckerTexture::new(
            Box::new(SolidColorTexture::new(Color::new(0.2, 0.3, 0.1))),
            Box::new(SolidColorTexture::new(Color::new(0.9, 0.9, 0.9))),
        );
        let material_ground = Arc::new(LambertianMaterial::new(Box::new(checker_texture)));
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
                        let albedo = Box::new(SolidColorTexture::new(Color::random()));
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
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            material_big1.clone(),
        )));
        world.push(Box::new(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            -0.95,
            material_big1,
        )));

        let material_big2 = Arc::new(DielectricMaterial::new(1.5));
        world.push(Box::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            material_big2,
        )));

        let material_big3 = Arc::new(MetalMaterial::new(Color::new(0.7, 0.6, 0.5), 0.0));
        world.push(Box::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            material_big3,
        )));

        BvhNode::new(world)
    }
}

pub struct TwoSphereCheckersScene;

impl Scene for TwoSphereCheckersScene {
    fn get_output_settings(&self) -> OutputSettings {
        OutputSettings::StaticImage {
            image_settings: ImageSettings {
                width: 854,
                height: 480,
                samples_per_pixel: 250,
                max_bounces: 20,
            },
        }
    }

    fn get_camera_at(&self, _: f64) -> Camera {
        let lookfrom = Vec3::new(13.0, 2.0, 3.0);
        let lookat = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let focus_dist = 10.0;
        let aperture = 0.0;
        let aspect_ratio = match self.get_output_settings() {
            OutputSettings::StaticImage { image_settings } => {
                image_settings.width as f64 / image_settings.height as f64
            }
            _ => unimplemented!(),
        };

        Camera::new(
            lookfrom,
            lookat,
            up,
            20.0,
            aspect_ratio,
            aperture,
            focus_dist,
        )
    }

    fn get_world(&self) -> BvhNode {
        let mut world: Vec<Box<dyn Hittable>> = vec![];

        let checker_texture = CheckerTexture::new(
            Box::new(SolidColorTexture::new(Color::new(0.2, 0.3, 0.1))),
            Box::new(SolidColorTexture::new(Color::new(0.9, 0.9, 0.9))),
        );
        let material_ground = Arc::new(LambertianMaterial::new(Box::new(checker_texture)));
        world.push(Box::new(Sphere::new(
            Vec3::new(0.0, -10.0, 0.0),
            10.0,
            material_ground.clone(),
        )));
        world.push(Box::new(Sphere::new(
            Vec3::new(0.0, 10.0, 0.0),
            10.0,
            material_ground,
        )));

        BvhNode::new(world)
    }
}
