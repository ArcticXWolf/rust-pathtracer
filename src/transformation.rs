use std::{
    fmt::Display,
    ops::{Mul, Neg},
};

use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4([[f64; 4]; 4]);

impl Matrix4x4 {
    pub fn new_scale_matrix_all_directions(scale: f64) -> Self {
        Matrix4x4::new_scale_matrix(scale, scale, scale)
    }

    pub fn new_scale_matrix(scale_x: f64, scale_y: f64, scale_z: f64) -> Self {
        Self([
            [scale_x, 0.0, 0.0, 0.0],
            [0.0, scale_y, 0.0, 0.0],
            [0.0, 0.0, scale_z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn new_identity_matrix() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn new_translation_matrix(translation: Vec3) -> Self {
        Self([
            [1.0, 0.0, 0.0, translation.x()],
            [0.0, 1.0, 0.0, translation.y()],
            [0.0, 0.0, 1.0, translation.z()],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn new_rotation_x_matrix(rotation_in_radians: f64) -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [
                0.0,
                rotation_in_radians.cos(),
                rotation_in_radians.sin(),
                0.0,
            ],
            [
                0.0,
                rotation_in_radians.sin().neg(),
                rotation_in_radians.cos(),
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn new_rotation_y_matrix(rotation_in_radians: f64) -> Self {
        Self([
            [
                rotation_in_radians.cos(),
                0.0,
                rotation_in_radians.sin().neg(),
                0.0,
            ],
            [0.0, 1.0, 0.0, 0.0],
            [
                rotation_in_radians.sin(),
                0.0,
                rotation_in_radians.cos(),
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn new_rotation_z_matrix(rotation_in_radians: f64) -> Self {
        Self([
            [
                rotation_in_radians.cos(),
                rotation_in_radians.sin().neg(),
                0.0,
                0.0,
            ],
            [
                rotation_in_radians.sin(),
                rotation_in_radians.cos(),
                0.0,
                0.0,
            ],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut r = Self([[0.0; 4]; 4]);
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    r.0[i][j] += self.0[i][k] * other.0[k][j];
                }
            }
        }
        r
    }
}

impl Mul<Vec3> for Matrix4x4 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.0[0][0] * rhs.x() + self.0[0][1] * rhs.y() + self.0[0][2] * rhs.z() + self.0[0][3],
            self.0[1][0] * rhs.x() + self.0[1][1] * rhs.y() + self.0[1][2] * rhs.z() + self.0[1][3],
            self.0[2][0] * rhs.x() + self.0[2][1] * rhs.y() + self.0[2][2] * rhs.z() + self.0[2][3],
        )
    }
}

impl Display for Matrix4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[{}, {}, {}, {}]",
            self.0[0][0], self.0[0][1], self.0[0][2], self.0[0][3]
        )?;
        writeln!(
            f,
            "[{}, {}, {}, {}]",
            self.0[1][0], self.0[1][1], self.0[1][2], self.0[1][3]
        )?;
        writeln!(
            f,
            "[{}, {}, {}, {}]",
            self.0[2][0], self.0[2][1], self.0[2][2], self.0[2][3]
        )?;
        writeln!(
            f,
            "[{}, {}, {}, {}]",
            self.0[3][0], self.0[3][1], self.0[3][2], self.0[3][3]
        )
    }
}
