use std::cmp::Ordering;

use rand::Rng;

use crate::{geometry::Hittable, ray::Ray, vec3::Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl Aabb {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        Self { minimum, maximum }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut found_t_min = t_min;
        let mut found_t_max = t_max;
        for axis_index in 0..3 {
            let inv_d = 1.0 / ray.direction.e[axis_index];
            let mut t0 = (self.minimum.e[axis_index] - ray.origin.e[axis_index]) * inv_d;
            let mut t1 = (self.maximum.e[axis_index] - ray.origin.e[axis_index]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            found_t_min = t0.max(found_t_min);
            found_t_max = t1.min(found_t_max);
            if found_t_max <= found_t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(&self, other: &Self) -> Self {
        Self::new(
            Vec3::new(
                self.minimum.x().min(other.minimum.x()),
                self.minimum.y().min(other.minimum.y()),
                self.minimum.z().min(other.minimum.z()),
            ),
            Vec3::new(
                self.maximum.x().max(other.maximum.x()),
                self.maximum.y().max(other.maximum.y()),
                self.maximum.z().max(other.maximum.z()),
            ),
        )
    }

    pub fn compare_axis(&self, other: &Self, axis: usize) -> Ordering {
        self.minimum.e[axis].total_cmp(&other.minimum.e[axis])
    }
}

#[derive(Clone)]
pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(source_objects: Vec<Box<dyn Hittable>>) -> Self {
        let mut objects = source_objects;
        let axis = rand::thread_rng().gen_range(0..3);
        match axis {
            0 => objects.sort_by(|a, b| a.bounding_box().compare_axis(&b.bounding_box(), 0)),
            1 => objects.sort_by(|a, b| a.bounding_box().compare_axis(&b.bounding_box(), 1)),
            2 => objects.sort_by(|a, b| a.bounding_box().compare_axis(&b.bounding_box(), 2)),
            _ => unreachable!(),
        }

        match objects.len() {
            0 | 1 => panic!("creating BVH node from empty object list"),
            2 => {
                let right = objects.pop().expect("no pop possible on length 2 vector?");
                let left = objects.pop().expect("no pop possible on length 1 vector?");
                let bbox = left.bounding_box().surrounding_box(&right.bounding_box());
                Self { left, right, bbox }
            }
            len => {
                let half_index = len / 2;
                let right_list = objects.split_off(half_index);

                let left = match objects.len() {
                    0 => panic!("empty list after split"),
                    1 => objects[0].clone(),
                    _ => Box::new(Self::new(objects)),
                };

                let right = match right_list.len() {
                    0 => panic!("empty list after split"),
                    1 => right_list[0].clone(),
                    _ => Box::new(Self::new(right_list.to_vec())),
                };

                let bbox = left.bounding_box().surrounding_box(&right.bounding_box());
                Self { left, right, bbox }
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<crate::geometry::HitRecord> {
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        let result = self.left.hit(ray, t_min, t_max);
        let new_t_max = match result {
            Some(ref hit_record) => hit_record.t,
            None => t_max,
        };
        match self.right.hit(ray, t_min, new_t_max) {
            Some(hit_record) => Some(hit_record),
            None => result,
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
