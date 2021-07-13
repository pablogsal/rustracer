use super::traits::Material;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)> {
        let scatter_dir = record.normal + Vec3::random_in_unit_sphere();
        // TODO: Check for NaN
        Some((self.albedo, Ray::new(record.p, scatter_dir)))
    }
}
