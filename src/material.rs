use std::ops::Neg;

use crate::{
    geometry::HitRecord,
    ray::Ray,
    texture::{SolidColorTexture, Texture},
    vec3::{Color, Vec3},
};

pub struct Scatter {
    pub scattered_ray: Ray,
    pub attenuation: Color,
}

pub trait Material: Sync + Send {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<Scatter> {
        None
    }
    fn emits(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Color {
        Color::default()
    }
}

pub struct LambertianMaterial {
    pub albedo: Box<dyn Texture>,
}

impl LambertianMaterial {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn new_from_color(color: Color) -> Self {
        Self {
            albedo: Box::new(SolidColorTexture::new(color)),
        }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = hit_record.normal + Vec3::random_on_unitsphere();

        if scatter_direction.near_zero() {
            println!("Near zero");
            scatter_direction = hit_record.normal;
        }

        Some(Scatter {
            scattered_ray: Ray::new(hit_record.point, scatter_direction),
            attenuation: self
                .albedo
                .value(hit_record.u, hit_record.v, hit_record.point),
        })
    }
}

pub struct MetalMaterial {
    pub albedo: Box<dyn Texture>,
    pub fuzz: f64,
}

impl MetalMaterial {
    pub fn new(albedo: Box<dyn Texture>, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }

    pub fn new_from_color(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo: Box::new(SolidColorTexture::new(albedo)),
            fuzz,
        }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let reflected_direction = ray_in.direction.unit_vector().reflect(hit_record.normal);

        if reflected_direction.dot(hit_record.normal) > 0.0 {
            Some(Scatter {
                scattered_ray: Ray::new(
                    hit_record.point,
                    reflected_direction + self.fuzz * Vec3::random_in_unitsphere(),
                ),
                attenuation: self
                    .albedo
                    .value(hit_record.u, hit_record.v, hit_record.point),
            })
        } else {
            None
        }
    }
}

pub struct DielectricMaterial {
    pub index_of_refraction: f64,
}

impl DielectricMaterial {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl DielectricMaterial {
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Schlick's approximation
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.direction.unit_vector();

        let cos_theta = unit_direction.neg().dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut direction = unit_direction;
        if (refraction_ratio * sin_theta > 1.0)
            || (DielectricMaterial::reflectance(cos_theta, refraction_ratio)
                > rand::random::<f64>())
        {
            direction = direction.reflect(hit_record.normal);
        } else {
            direction = direction.refract(hit_record.normal, refraction_ratio);
        }

        Some(Scatter {
            scattered_ray: Ray::new(hit_record.point, direction),
            attenuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}

pub struct DiffuseLightMaterial {
    pub emit: Box<dyn Texture>,
}

impl DiffuseLightMaterial {
    pub fn new_from_color(color: Color) -> Self {
        Self {
            emit: Box::new(SolidColorTexture::new(color)),
        }
    }
}

impl Material for DiffuseLightMaterial {
    fn emits(&self, _: &Ray, hit_record: &HitRecord) -> Color {
        if hit_record.front_face {
            self.emit
                .value(hit_record.u, hit_record.v, hit_record.point)
        } else {
            Color::default()
        }
    }
}
