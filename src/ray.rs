use crate::vec3::Vec3;
use crate::vec3::Vec3 as Point;

/// Ray is a function in a form: `P(t) = A + tb`, where A is an origin, t is a parameter and
/// b is a direction
#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    /// Get value of point `P(t) = A + direction * t`
    pub fn at(&self, t: f64) -> Point {
        self.origin + t * &self.direction
    }

    pub fn unit_vector(&self) -> Vec3 {
        self.direction / self.direction.length()
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn origin(&self) -> Point {
        self.origin
    }
}
