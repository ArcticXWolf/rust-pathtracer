use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use rand::Rng;

const NEAR_ZERO: f64 = 1e-8;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub e: [f64; 2],
}

impl Default for Vec2 {
    fn default() -> Self {
        Self { e: [0.0, 0.0] }
    }
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { e: [x, y] }
    }

    pub fn random() -> Self {
        Self::new(rand::random::<f64>(), rand::random::<f64>())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
        )
    }

    pub fn random_in_unitsphere() -> Self {
        // rejection sampling
        loop {
            let p = Self::random_range(-1.0, 1.0);
            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_on_unitsphere() -> Self {
        Self::random_in_unitsphere().unit_vector()
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y()
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y()
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }

    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        Self::new(f(self.x()), f(self.y()))
    }

    pub fn near_zero(&self) -> bool {
        self.x().abs() < NEAR_ZERO && self.y().abs() < NEAR_ZERO
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.x();
        self.e[1] += rhs.y();
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0] -= rhs.x();
        self.e[1] -= rhs.y();
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x() * rhs.x(), self.y() * rhs.y())
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.e[0] *= rhs.x();
        self.e[1] *= rhs.y();
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        self.map(|v| v * t)
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, vec: Vec2) -> Self::Output {
        vec.map(|v| v * self)
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x() / rhs.x(), self.y() / rhs.y())
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.e[0] /= rhs.x();
        self.e[1] /= rhs.y();
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        self.map(|v| v / t)
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
    }
}

impl Index<usize> for Vec2 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(|v| v * -1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);
        let c = Vec2::new(1.0, 1.0);
        let d = Vec2::new(0.0, 10.0);

        assert_eq!([1.0, 1.0], (a + b).e);
        assert_eq!([1.0, -1.0], (a - b).e);
        assert_eq!([-1.0, -1.0], (-c).e);
    }
}
