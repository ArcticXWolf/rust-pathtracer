use std::{ops::Neg, sync::Arc};

use crate::{
    bvh::BvhNode,
    camera::Camera,
    geometry::{AABox, Hittable, RectangleXY, RectangleXZ, RectangleYZ, Sphere, Triangle},
    material::{
        DielectricMaterial, DiffuseLightMaterial, LambertianMaterial, Material, MetalMaterial,
    },
    texture::{CheckerTexture, PerlinNoiseTexture, SolidColorTexture},
    vec3::{Color, Vec3},
};

pub struct ImageSettings {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub max_bounces: usize,
    pub background: Color,
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
                background: Color::new(1.0, 1.0, 1.0),
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
                background: Color::new(1.0, 1.0, 1.0),
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
            material_ground,
        )));

        let perlin_texture = PerlinNoiseTexture::new(4.0);
        let material_top = Arc::new(LambertianMaterial::new(Box::new(perlin_texture)));
        world.push(Box::new(Sphere::new(
            Vec3::new(0.0, 10.0, 0.0),
            10.0,
            material_top,
        )));

        BvhNode::new(world)
    }
}

pub struct LightTestScene;

impl Scene for LightTestScene {
    fn get_output_settings(&self) -> OutputSettings {
        OutputSettings::StaticImage {
            image_settings: ImageSettings {
                width: 854,
                height: 480,
                samples_per_pixel: 2000,
                max_bounces: 50,
                background: Color::new(0.0, 0.0, 0.0),
            },
        }
    }

    fn get_camera_at(&self, _: f64) -> Camera {
        let lookfrom = Vec3::new(26.0, 3.0, 6.0);
        let lookat = Vec3::new(0.0, 2.0, 0.0);
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

        let perlin_texture = PerlinNoiseTexture::new(4.0);
        let material_ground = Arc::new(LambertianMaterial::new(Box::new(perlin_texture)));
        world.push(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            material_ground.clone(),
        )));
        world.push(Box::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            material_ground,
        )));

        let material_light = Arc::new(DiffuseLightMaterial::new_from_color(Color::new(
            4.0, 4.0, 4.0,
        )));
        world.push(Box::new(
            RectangleXY::new(
                Vec3::new(3.0, 1.0, -2.0),
                Vec3::new(5.0, 3.0, -2.0),
                1.0,
                material_light.clone(),
            )
            .expect("rectangle definition is not axis aligned"),
        ));
        world.push(Box::new(
            RectangleXZ::new(
                Vec3::new(-1.0, 6.0, -1.0),
                Vec3::new(1.0, 6.0, 1.0),
                -1.0,
                material_light.clone(),
            )
            .expect("rectangle definition is not axis aligned"),
        ));
        world.push(Box::new(
            RectangleYZ::new(
                Vec3::new(-6.0, 1.0, -2.0),
                Vec3::new(-6.0, 3.0, 2.0),
                1.0,
                material_light,
            )
            .expect("rectangle definition is not axis aligned"),
        ));

        BvhNode::new(world)
    }
}

pub struct CornellBoxScene;

impl Scene for CornellBoxScene {
    fn get_output_settings(&self) -> OutputSettings {
        OutputSettings::StaticImage {
            image_settings: ImageSettings {
                width: 400,
                height: 400,
                samples_per_pixel: 1000,
                max_bounces: 20,
                background: Color::new(0.0, 0.0, 0.0),
            },
        }
    }

    fn get_camera_at(&self, _: f64) -> Camera {
        let lookfrom = Vec3::new(278.0, 278.0, -800.0);
        let lookat = Vec3::new(278.0, 278.0, 0.0);
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
            40.0,
            aspect_ratio,
            aperture,
            focus_dist,
        )
    }

    fn get_world(&self) -> BvhNode {
        let mut world: Vec<Box<dyn Hittable>> = vec![];

        let material_red = Arc::new(LambertianMaterial::new_from_color(Color::new(
            0.65, 0.05, 0.05,
        )));
        let material_white = Arc::new(LambertianMaterial::new_from_color(Color::new(
            0.73, 0.73, 0.73,
        )));
        let material_green = Arc::new(LambertianMaterial::new_from_color(Color::new(
            0.12, 0.45, 0.15,
        )));
        let material_light = Arc::new(DiffuseLightMaterial::new_from_color(Color::new(
            15.0, 15.0, 15.0,
        )));
        let material_glass = Arc::new(DielectricMaterial::new(1.5));

        world.push(Box::new(
            RectangleYZ::new(
                Vec3::new(555.0, 0.0, 0.0),
                Vec3::new(555.0, 555.0, 555.0),
                -1.0,
                material_green,
            )
            .expect("rectangle definition is not axis aligned"),
        ));
        world.push(Box::new(
            RectangleYZ::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 555.0, 555.0),
                1.0,
                material_red,
            )
            .expect("rectangle definition is not axis aligned"),
        ));

        world.push(Box::new(
            RectangleXZ::new(
                Vec3::new(0.0, 555.0, 0.0),
                Vec3::new(555.0, 555.0, 555.0),
                -1.0,
                material_white.clone(),
            )
            .expect("rectangle definition is not axis aligned"),
        ));
        world.push(Box::new(
            RectangleXZ::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(555.0, 0.0, 555.0),
                1.0,
                material_white.clone(),
            )
            .expect("rectangle definition is not axis aligned"),
        ));
        world.push(Box::new(
            RectangleXZ::new(
                Vec3::new(213.0, 554.0, 227.0),
                Vec3::new(343.0, 554.0, 332.0),
                -1.0,
                material_light,
            )
            .expect("rectangle definition is not axis aligned"),
        ));

        world.push(Box::new(
            RectangleXY::new(
                Vec3::new(0.0, 0.0, 555.0),
                Vec3::new(555.0, 555.0, 555.0),
                -1.0,
                material_white.clone(),
            )
            .expect("rectangle definition is not axis aligned"),
        ));

        world.push(Box::new(AABox::new(
            Vec3::new(130.0, 0.0, 65.0),
            Vec3::new(295.0, 165.0, 230.0),
            material_white.clone(),
        )));
        world.push(Box::new(AABox::new(
            Vec3::new(265.0, 0.0, 295.0),
            Vec3::new(430.0, 330.0, 460.0),
            material_white,
        )));

        world.push(Box::new(Sphere::new(
            Vec3::new(212.5, 255.0, 147.5),
            90.0,
            material_glass.clone(),
        )));
        world.push(Box::new(Sphere::new(
            Vec3::new(347.5, 420.0, 377.5),
            90.0,
            material_glass,
        )));

        BvhNode::new(world)
    }
}

pub struct TriangleTestScene;

impl Scene for TriangleTestScene {
    fn get_output_settings(&self) -> OutputSettings {
        OutputSettings::StaticImage {
            image_settings: ImageSettings {
                width: 400,
                height: 400,
                samples_per_pixel: 1000,
                max_bounces: 20,
                background: Color::new(0.0, 0.0, 0.0),
            },
        }
    }

    fn get_camera_at(&self, _: f64) -> Camera {
        let lookfrom = Vec3::new(278.0, 278.0, -800.0);
        let lookat = Vec3::new(278.0, 278.0, 0.0);
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
            40.0,
            aspect_ratio,
            aperture,
            focus_dist,
        )
    }

    fn get_world(&self) -> BvhNode {
        let mut world: Vec<Box<dyn Hittable>> = vec![];

        let material_red = Arc::new(LambertianMaterial::new_from_color(Color::new(
            0.65, 0.05, 0.05,
        )));
        let material_white = Arc::new(LambertianMaterial::new_from_color(Color::new(
            0.73, 0.73, 0.73,
        )));
        let material_green = Arc::new(LambertianMaterial::new_from_color(Color::new(
            0.12, 0.45, 0.15,
        )));
        let material_light = Arc::new(DiffuseLightMaterial::new_from_color(Color::new(
            15.0, 15.0, 15.0,
        )));
        let material_glass = Arc::new(DielectricMaterial::new(1.5));

        world.push(Box::new(
            RectangleYZ::new(
                Vec3::new(555.0, 0.0, 0.0),
                Vec3::new(555.0, 555.0, 555.0),
                -1.0,
                material_green,
            )
            .expect("rectangle definition is not axis aligned"),
        ));
        world.push(Box::new(
            RectangleYZ::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 555.0, 555.0),
                1.0,
                material_red,
            )
            .expect("rectangle definition is not axis aligned"),
        ));

        world.push(Box::new(
            RectangleXZ::new(
                Vec3::new(0.0, 555.0, 0.0),
                Vec3::new(555.0, 555.0, 555.0),
                -1.0,
                material_white.clone(),
            )
            .expect("rectangle definition is not axis aligned"),
        ));
        world.push(Box::new(
            RectangleXZ::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(555.0, 0.0, 555.0),
                1.0,
                material_white.clone(),
            )
            .expect("rectangle definition is not axis aligned"),
        ));
        world.push(Box::new(
            RectangleXZ::new(
                Vec3::new(213.0, 554.0, 227.0),
                Vec3::new(343.0, 554.0, 332.0),
                -1.0,
                material_light,
            )
            .expect("rectangle definition is not axis aligned"),
        ));

        world.push(Box::new(
            RectangleXY::new(
                Vec3::new(0.0, 0.0, 555.0),
                Vec3::new(555.0, 555.0, 555.0),
                -1.0,
                material_white.clone(),
            )
            .expect("rectangle definition is not axis aligned"),
        ));

        world.push(Box::new(Triangle::new_without_normal(
            Vec3::new(200.0, 100.0, 100.0),
            Vec3::new(300.0, 300.0, 500.0),
            Vec3::new(400.0, 100.0, 100.0),
            material_glass,
        )));
        world.push(Box::new(Triangle::new_without_normal(
            Vec3::new(100.0, 300.0, 100.0),
            Vec3::new(150.0, 400.0, 250.0),
            Vec3::new(100.0, 300.0, 400.0),
            material_white,
        )));

        BvhNode::new(world)
    }
}
