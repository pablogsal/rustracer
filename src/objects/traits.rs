use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a Box<dyn Material>,
}

impl<'a> HitRecord<'_> {
    pub fn new(
        ray: &Ray,
        p: Point3,
        outward_normal: Vec3,
        t: f64,
        material: &'a Box<dyn Material>,
    ) -> HitRecord<'a> {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
