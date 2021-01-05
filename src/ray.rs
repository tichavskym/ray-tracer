use crate::vec3::Vec3;

/// Ray is a function in a form: `P(t) = A + tb`, where A is an origin, t is a parameter and
/// b is a direction
#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    /// Get value of point `P(t) = A + direction * t`
    pub fn at(&self, t: f64) -> Vec3 {
        &self.origin + &(t * &self.direction)
    }

    pub fn unit_vector(&self) -> Vec3 {
        &self.direction / self.direction.length()
    }

    pub fn direction(&self) -> Vec3 {
        self.direction.clone()
    }

    pub fn origin(&self) -> Vec3 {
        self.origin.clone()
    }
}
