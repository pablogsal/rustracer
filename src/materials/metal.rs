use super::traits::Material;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Metal {
        if fuzziness > 1.0 {
            Metal {
                albedo,
                fuzziness: 1.0,
            }
        } else if fuzziness < 0.0 {
            Metal {
                albedo,
                fuzziness: 0.0,
            }
        } else {
            Metal { albedo, fuzziness }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray.direction().normalize().reflect(&record.normal);
        let scattered = Ray::new(
            record.p,
            reflected + self.fuzziness * Vec3::random_unit_vector(),
        );

        if scattered.direction().dot(&record.normal) < 0.0 {
            return None;
        }

        Some((self.albedo, scattered))
    }
}
