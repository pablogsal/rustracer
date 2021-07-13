use super::traits::Material;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Vec3;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r02 = r0 * r0;
        r02 + (1.0 - r02) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)> {
        let refraction_ratio = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index / 1.0
        };

        let direction_normalized = ray.direction().normalize();

        let cos_thetha = -direction_normalized.dot(&record.normal).min(1.0);
        let sin_thetha = (1.0 - cos_thetha * cos_thetha).sqrt();

        let mut rng = rand::thread_rng();
        let final_direction = if refraction_ratio * sin_thetha > 1.0
            || self.reflectance(cos_thetha, refraction_ratio) > rng.gen()
        {
            // Cannot refract
            direction_normalized.reflect(&record.normal)
        } else {
            // A refraction is possible
            direction_normalized.refract(&record.normal, refraction_ratio)
        };

        let attenuation = Color::new(0.95, 0.95, 0.95);
        Some((attenuation, Ray::new(record.p, final_direction)))
    }
}
