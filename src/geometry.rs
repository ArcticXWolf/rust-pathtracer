use std::sync::Arc;

use crate::{material::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord<'a> {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f64,
        point: Vec3,
        ray: &Ray,
        outward_normal: Vec3,
        material: &'a dyn Material,
    ) -> HitRecord<'a> {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        HitRecord {
            t,
            point,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            front_face,
            material,
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = ray.direction.dot(oc);
        let c = oc.len_squared() - self.radius * self.radius;
        let dis = half_b * half_b - a * c;
        if dis < 0.0 {
            return None;
        }

        let mut root = (-half_b - dis.sqrt()) / a;
        if root < t_min || root > t_max {
            root = (-half_b + dis.sqrt()) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;

        Some(HitRecord::new(
            root,
            point,
            ray,
            outward_normal,
            &*self.material,
        ))
    }
}
