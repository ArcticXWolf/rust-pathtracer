use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        up: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let h = (vertical_fov.to_radians() / 2.0).tan();
        let viewport = (aspect_ratio * 2.0 * h, 2.0 * h);

        let w = (lookfrom - lookat).unit_vector();
        let u = up.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport.0 * u;
        let vertical = focus_dist * viewport.1 * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn ray_at(&self, s: f64, t: f64) -> Ray {
        let rng = self.lens_radius * Vec3::random_in_unitdisk_xy();
        let blur_offset = self.u * rng.x() + self.v * rng.y();

        Ray::new(
            self.origin + blur_offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - blur_offset,
        )
    }
}
