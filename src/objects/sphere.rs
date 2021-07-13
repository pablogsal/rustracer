use super::traits::HitRecord;
use super::traits::Hittable;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_center = *ray.origin() - self.center;

        let a = ray.direction().length_squared();
        let half_b = origin_center.dot(ray.direction());
        let c = origin_center.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            // The ray doesn't intersect the sphere
            return None;
        }

        // There is at least one root: Find the nearest root that lies in
        // the acceptable range provided by the caller.

        let sqrtd = discriminant.sqrt();

        // Consider the first root (the smaller one)
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            // Consider the other root
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        Some(HitRecord::new(ray, p, outward_normal, root, &self.material))
    }
}
