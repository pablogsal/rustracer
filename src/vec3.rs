use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;
use std::usize;

use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            e: [rng.gen(), rng.gen(), rng.gen()],
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            e: [
                rng.gen_range(min..max),
                rng.gen_range(min..max),
                rng.gen_range(min..max),
            ],
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().normalize()
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn normalize(&self) -> Vec3 {
        self.clone() / self.length()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        return *self - 2.0 * self.dot(normal) * (*normal);
    }

    pub fn refract(&self, normal: &Vec3, ref_ratio: f64) -> Vec3 {
        let cos_theta = -self.dot(normal).min(1.0);
        let r_out_perp = ref_ratio * (*self + cos_theta * *normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *normal;
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Vec3 {
        Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Vec3 {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Vec3 {
        Vec3::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(v.x() * self, v.y() * self, v.z() * self)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_vectors() {
        let x = Vec3::new(1.0, 1.0, 1.0);
        let y = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(x + y, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn add_inplace() {
        let mut x = Vec3::new(1.0, 1.0, 1.0);
        x += Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(x, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn sub_two_vectors() {
        let x = Vec3::new(1.0, 1.0, 1.0);
        let y = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(x - y, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn sub_inplace() {
        let mut x = Vec3::new(1.0, 1.0, 1.0);
        x -= Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(x, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn mul_two_vectors() {
        let x = Vec3::new(2.0, 2.0, 2.0);
        let y = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(x + y, Vec3::new(4.0, 4.0, 4.0));
    }

    #[test]
    fn mul_inplace() {
        let mut x = Vec3::new(2.0, 2.0, 2.0);
        x += Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(x, Vec3::new(4.0, 4.0, 4.0));
    }

    #[test]
    fn normalize() {
        let x = Vec3::new(10.0, 10.0, 10.0);
        let x_norm = x.normalize();
        assert_eq!(x_norm.x(), x_norm.y());
        assert_eq!(x_norm.x(), x_norm.z());
        assert!((x.normalize().z() - 0.57735026).abs() < 0.000001);
    }

    #[test]
    fn length() {
        let x = Vec3::new(10.0, 10.0, 10.0);
        assert!((x.length() - 17.320509).abs() < 0.000001);
    }

    #[test]
    fn dot_product() {
        let x = Vec3::new(2.0, 2.0, 2.0);
        let y = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(x.dot(&y), 18.0);
    }

    #[test]
    fn cross_product() {
        let x = Vec3::new(2.0, 2.0, 2.0);
        let y = Vec3::new(-3.0, 3.0, -1.0);
        assert_eq!(x.cross(&y), Vec3::new(-8.0, -4.0, 8.0));
    }
}
