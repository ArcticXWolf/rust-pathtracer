use std::sync::Arc;

use crate::{bvh::Aabb, material::Material, ray::Ray, vec3::Vec3};

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

pub trait Hittable: Sync + Send + CloneHittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}

trait CloneHittable {
    fn clone_hittable<'a>(&self) -> Box<dyn Hittable>;
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
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn push(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut possible_hit: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                possible_hit = Some(hit);
            }
        }

        possible_hit
    }

    fn bounding_box(&self) -> Aabb {
        if let Some(first_object) = self.objects.first() {
            let mut containing_box = first_object.bounding_box();

            for object in self.objects.iter().skip(1) {
                let obj_box = object.bounding_box();
                containing_box = containing_box.surrounding_box(&obj_box);
            }
            return containing_box;
        }
        panic!("called bounding box of empty Hittable list");
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

    fn bounding_box(&self) -> Aabb {
        Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        )
    }
}
