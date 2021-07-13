use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}
