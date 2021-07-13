mod sphere;
mod traits;

pub use sphere::*;
pub use traits::*;

type HittableVec = Vec<Box<dyn Hittable>>;
pub struct HittableCollection {
    objects: HittableVec,
}

impl HittableCollection {
    pub fn new() -> HittableCollection {
        HittableCollection { objects: vec![] }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }
}

impl Hittable for HittableCollection {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.objects
            .iter()
            .flat_map(|obj| obj.hit(ray, t_min, t_max))
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }
}
