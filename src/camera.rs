use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::Vec3 as Point;

/// Image sensor (imager) parameters:
/// `Focal length` is a distance between projection plane to projection point (origin).
/// `origin` and `lower_left_corner` together with `focal_length` give us spacial orientation of
/// the virtual sensor.
pub struct Sensor {
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point,
}

impl Sensor {
    pub fn new(height: f64, aspect_ratio: f64, focal_length: f64) -> Sensor {
        let origin = Point::zero();
        let width = aspect_ratio * height;
        let horizontal = Vec3::new(width, 0., 0.);
        let vertical = Vec3::new(0., height, 0.);

        Sensor {
            origin,
            horizontal,
            vertical,
            lower_left_corner: &origin
                - &(&horizontal / 2.0)
                - &vertical / 2.0
                - Vec3::new(0., 0., focal_length),
        }
    }

    // The ray goes from origin to the pixel in the virtual viewport which is specified by offset
    // vectors `u` and `v`.
    pub fn calculate_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            &(&self.lower_left_corner + &(u as f64 * &self.horizontal))
                + &(v as f64 * &self.vertical)
                - self.origin,
        )
    }
}
