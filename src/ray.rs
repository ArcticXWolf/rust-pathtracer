use crate::{
    geometry::Hittable,
    vec3::{Color, Vec3},
};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn color(&self, hittable: &impl Hittable, background: Color, bounces_left: usize) -> Color {
        if bounces_left == 0 {
            return Color::default();
        }

        if let Some(hit_record) = hittable.hit(self, 0.001, f64::INFINITY) {
            let emitted = hit_record
                .material
                .emits(hit_record.u, hit_record.v, hit_record.point);

            if let Some(scatter) = hit_record.material.scatter(self, &hit_record) {
                return emitted
                    + scatter.attenuation
                        * scatter
                            .scattered_ray
                            .color(hittable, background, bounces_left - 1);
            }

            return emitted;
        }

        // Background
        background
    }
}
