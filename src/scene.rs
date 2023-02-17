use std::{
    io::Read,
    ops::Neg,
    path::{self, Path},
    sync::Arc,
};

use crate::{
    bvh::BvhNode,
    camera::Camera,
    geometry::{AABox, Hittable, RectangleXY, RectangleXZ, RectangleYZ, Sphere, Triangle},
    material::{
        DielectricMaterial, DiffuseLightMaterial, LambertianMaterial, Material, MetalMaterial,
    },
    obj_model::ObjModel,
    parser::{self, NamedToken, NamedTokenType, Token, Value},
    texture::{CheckerTexture, PerlinNoiseTexture, SolidColorTexture},
    transformation::Matrix4x4,
    vec3::{Color, Vec3},
};

#[derive(Debug, Clone)]
pub struct ImageSettings {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub max_bounces: usize,
    pub background: Color,
    pub file_name: String,
}

#[derive(Debug, Clone)]
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

impl Default for OutputSettings {
    fn default() -> Self {
        OutputSettings::StaticImage {
            image_settings: ImageSettings {
                width: 0,
                height: 0,
                samples_per_pixel: 1000,
                max_bounces: 10,
                background: Color::new(0.0, 0.0, 0.0),
                file_name: String::from(""),
            },
        }
    }
}

pub trait Scene {
    fn get_world(&self) -> BvhNode;
    fn get_camera_at(&self, t: f64) -> Camera;
    fn get_output_settings(&self) -> OutputSettings;
}

pub struct PbrtScene {
    output_settings: OutputSettings,
    camera: Camera,
    world: BvhNode,
}

impl PbrtScene {
    pub fn new_from_file(path: &Path) -> Self {
        let mut f = std::fs::File::open(path).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        let (_, parsed) = parser::parse::<nom::error::VerboseError<&str>>(&contents).unwrap();

        let mut output_settings = OutputSettings::default();
        let mut camera = Camera::default();
        let mut world: Vec<Box<dyn Hittable>> = vec![];

        for token in parsed {
            match token {
                Token::LookAt { eye, look, up } => {
                    camera.lookfrom = eye;
                    camera.lookat = look;
                    camera.up = up;
                }
                Token::NamedToken(nt) => match nt.object_type {
                    NamedTokenType::Camera => {
                        for (value_name, value) in nt.values {
                            match (value_name.as_str(), value) {
                                ("fov", Value::Float(fov)) => {
                                    camera.vertical_fov = *fov.first().unwrap()
                                }
                                _ => {}
                            }
                        }
                    }
                    NamedTokenType::Film => {
                        for (value_name, value) in nt.values {
                            match (value_name.as_str(), value) {
                                ("filename", Value::String(string)) => {
                                    output_settings.
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Self {
            output_settings,
            camera,
            world: BvhNode::new(world),
        }
    }
}

impl Scene for PbrtScene {
    fn get_output_settings(&self) -> OutputSettings {
        self.output_settings.clone()
    }

    fn get_camera_at(&self, t: f64) -> Camera {
        self.camera.clone()
    }

    fn get_world(&self) -> BvhNode {
        self.world.clone()
    }
}
