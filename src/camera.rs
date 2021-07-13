use crate::ray::*;
use crate::vec3::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    basis: [Vec3; 3],
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Point3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Camera {
        let theta = vertical_fov / 180.0 * std::f64::consts::PI;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // The basis here will be (u,v,w) describing 3 orthonormal vectors
        // that set the camera position. w points in the oposite direction
        // that the camera is looking at, while v and u spawn the camera
        // plane subspace.

        // This means that w will be defined as the normal vector to the camera plane.

        // To obtain u we take the cross-product of an arbitrary "up" direction
        // for the camera and the w vector. This serves to define a perpendicular
        // direction and fully defines the camera plane (because we know its normal and
        // u is one of the vectors that spawns it). Then we can simply cross w and u to get the
        // mising vector that spawns the camera plane.

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w,
            horizontal,
            vertical,
            lens_radius,
            basis: [u, v, w],
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let random_point = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.basis[0] * random_point.x() + self.basis[1] * random_point.y();
        let origin = self.origin + offset;
        return Ray::new(
            origin,
            (self.lower_left_corner + u * self.horizontal + v * self.vertical) - origin,
        );
    }
}
