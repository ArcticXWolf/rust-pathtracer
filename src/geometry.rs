use std::{ops::Neg, sync::Arc};

use crate::{bvh::Aabb, material::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord<'a> {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f64,
        point: Vec3,
        ray: &Ray,
        outward_normal: Vec3,
        u: f64,
        v: f64,
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
            u,
            v,
            front_face,
            material,
        }
    }
}

pub trait Hittable: Sync + Send + CloneHittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}

pub trait CloneHittable {
    fn clone_hittable(&self) -> Box<dyn Hittable>;
}

impl<T> CloneHittable for T
where
    T: Hittable + Clone + 'static,
{
    fn clone_hittable(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Self {
        self.clone_hittable()
    }
}

#[derive(Clone)]
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

    fn get_sphere_uv(point: Vec3) -> (f64, f64) {
        let theta = point.y().neg().acos();
        let phi = point.z().neg().atan2(point.x()) + std::f64::consts::PI;

        (
            phi / (2.0 * std::f64::consts::PI),
            theta / std::f64::consts::PI,
        )
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
        let (u, v) = Sphere::get_sphere_uv(outward_normal);

        Some(HitRecord::new(
            root,
            point,
            ray,
            outward_normal,
            u,
            v,
            &*self.material,
        ))
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::new(
            self.center - Vec3::new(self.radius.abs(), self.radius.abs(), self.radius.abs()),
            self.center + Vec3::new(self.radius.abs(), self.radius.abs(), self.radius.abs()),
        )
    }
}
