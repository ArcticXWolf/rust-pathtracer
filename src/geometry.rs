use std::{fmt, ops::Neg, sync::Arc};

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
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
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
        let (u, v) = Self::get_sphere_uv(outward_normal);

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

#[derive(Debug)]
pub struct RectangleNotAxisAlignedError;

impl fmt::Display for RectangleNotAxisAlignedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rectangle axis is not aligned")
    }
}

#[derive(Clone)]
pub struct RectangleXY {
    start: Vec3,
    end: Vec3,
    material: Arc<dyn Material>,
}

impl RectangleXY {
    pub fn new(
        start: Vec3,
        end: Vec3,
        material: Arc<dyn Material>,
    ) -> Result<Self, RectangleNotAxisAlignedError> {
        if start.z() != end.z() {
            return Err(RectangleNotAxisAlignedError);
        }
        Ok(Self {
            start: Vec3::new(start.x().min(end.x()), start.y().min(end.y()), start.z()),
            end: Vec3::new(start.x().max(end.x()), start.y().max(end.y()), start.z()),
            material,
        })
    }
}

impl Hittable for RectangleXY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.start.z() - ray.origin.z()) / ray.direction.z();
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x() + t * ray.direction.x();
        let y = ray.origin.y() + t * ray.direction.y();

        if x < self.start.x() || x > self.end.x() || y < self.start.y() || y > self.end.y() {
            return None;
        }

        Some(HitRecord::new(
            t,
            ray.at(t),
            ray,
            Vec3::new(0.0, 0.0, 1.0),
            (x - self.start.x()) / (self.end.x() - self.start.x()),
            (y - self.start.y()) / (self.end.y() - self.start.y()),
            &*self.material,
        ))
    }

    fn bounding_box(&self) -> Aabb {
        // In theory this bounding box is infinitly thin, but we pad it a bit.
        // Also, using start in both arguments is correct, since we want to
        // force the axis alignment.
        Aabb::new(
            Vec3::new(self.start.x(), self.start.y(), self.start.z() - 0.0001),
            Vec3::new(self.end.x(), self.end.y(), self.start.z() + 0.0001),
        )
    }
}

#[derive(Clone)]
pub struct RectangleXZ {
    start: Vec3,
    end: Vec3,
    material: Arc<dyn Material>,
}

impl RectangleXZ {
    pub fn new(
        start: Vec3,
        end: Vec3,
        material: Arc<dyn Material>,
    ) -> Result<Self, RectangleNotAxisAlignedError> {
        if start.y() != end.y() {
            return Err(RectangleNotAxisAlignedError);
        }
        Ok(Self {
            start: Vec3::new(start.x().min(end.x()), start.y(), start.z().min(end.z())),
            end: Vec3::new(start.x().max(end.x()), start.y(), start.z().max(end.z())),
            material,
        })
    }
}

impl Hittable for RectangleXZ {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.start.y() - ray.origin.y()) / ray.direction.y();
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();

        if x < self.start.x() || x > self.end.x() || z < self.start.z() || z > self.end.z() {
            return None;
        }

        Some(HitRecord::new(
            t,
            ray.at(t),
            ray,
            Vec3::new(0.0, 1.0, 0.0),
            (x - self.start.x()) / (self.end.x() - self.start.x()),
            (z - self.start.z()) / (self.end.z() - self.start.z()),
            &*self.material,
        ))
    }

    fn bounding_box(&self) -> Aabb {
        // In theory this bounding box is infinitly thin, but we pad it a bit.
        // Also, using start in both arguments is correct, since we want to
        // force the axis alignment.
        Aabb::new(
            Vec3::new(self.start.x(), self.start.y() - 0.0001, self.start.z()),
            Vec3::new(self.end.x(), self.start.y() + 0.0001, self.end.z()),
        )
    }
}

#[derive(Clone)]
pub struct RectangleYZ {
    start: Vec3,
    end: Vec3,
    material: Arc<dyn Material>,
}

impl RectangleYZ {
    pub fn new(
        start: Vec3,
        end: Vec3,
        material: Arc<dyn Material>,
    ) -> Result<Self, RectangleNotAxisAlignedError> {
        if start.x() != end.x() {
            return Err(RectangleNotAxisAlignedError);
        }
        Ok(Self {
            start: Vec3::new(start.x(), start.y().min(end.y()), start.z().min(end.z())),
            end: Vec3::new(start.x(), start.y().max(end.y()), start.z().max(end.z())),
            material,
        })
    }
}

impl Hittable for RectangleYZ {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.start.x() - ray.origin.x()) / ray.direction.x();
        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin.y() + t * ray.direction.y();
        let z = ray.origin.z() + t * ray.direction.z();

        if y < self.start.y() || y > self.end.y() || z < self.start.z() || z > self.end.z() {
            return None;
        }

        Some(HitRecord::new(
            t,
            ray.at(t),
            ray,
            Vec3::new(1.0, 0.0, 0.0),
            (y - self.start.y()) / (self.end.y() - self.start.y()),
            (z - self.start.z()) / (self.end.z() - self.start.z()),
            &*self.material,
        ))
    }

    fn bounding_box(&self) -> Aabb {
        // In theory this bounding box is infinitly thin, but we pad it a bit.
        // Also, using start in both arguments is correct, since we want to
        // force the axis alignment.
        Aabb::new(
            Vec3::new(self.start.x() - 0.0001, self.start.y(), self.start.z()),
            Vec3::new(self.start.x() + 0.0001, self.end.y(), self.end.z()),
        )
    }
}
