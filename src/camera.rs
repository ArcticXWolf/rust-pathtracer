use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Clone)]
pub struct Camera {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub up: Vec3,
    pub vertical_fov: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus_dist: f64,

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
        let mut camera = Self {
            lookfrom,
            lookat,
            up,
            vertical_fov,
            aspect_ratio,
            aperture,
            focus_dist,
            lower_left_corner: Vec3::default(),
            horizontal: Vec3::default(),
            vertical: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            lens_radius: 0.0,
        };
        camera.recalculate();
        camera
    }

    pub fn recalculate(&mut self) {
        let h = (self.vertical_fov.to_radians() / 2.0).tan();
        let viewport = (self.aspect_ratio * 2.0 * h, 2.0 * h);

        let w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.up.cross(w).unit_vector();
        self.v = w.cross(self.u);

        self.horizontal = self.focus_dist * viewport.0 * self.u;
        self.vertical = self.focus_dist * viewport.1 * self.v;
        self.lower_left_corner =
            self.lookfrom - self.horizontal / 2.0 - self.vertical / 2.0 - self.focus_dist * w;
    }

    pub fn ray_at(&self, s: f64, t: f64) -> Ray {
        let rng = self.lens_radius * Vec3::random_in_unitdisk_xy();
        let blur_offset = self.u * rng.x() + self.v * rng.y();

        Ray::new(
            self.lookfrom + blur_offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.lookfrom
                - blur_offset,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Vec3::default(),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            16.0 / 9.0,
            0.0,
            10.0,
        )
    }
}
