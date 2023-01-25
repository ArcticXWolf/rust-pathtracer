use crate::{
    geometry::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

pub struct Scatter {
    pub scattered_ray: Ray,
    pub attenuation: Color,
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}

pub struct LambertianMaterial {
    pub albedo: Color,
}

impl LambertianMaterial {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = hit_record.normal + Vec3::random_on_unitsphere();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some(Scatter {
            scattered_ray: Ray::new(hit_record.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct MetalMaterial {
    pub albedo: Color,
    pub fuzz: f64,
}

impl MetalMaterial {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
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
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
