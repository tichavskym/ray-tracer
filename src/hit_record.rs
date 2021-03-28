use crate::vec3::Vec3;
use crate::vec3::Vec3 as Point; // For easier understanding

// Stores information about intersection of ray and the object.
pub struct HitRecord {
    // Point of intersection.
    pub(crate) point: Point,
    // Normal surface vector at the point of intersection.
    pub(crate) normal: Vec3,
    // Parameter that says where on the ray the intersection happened.
    pub(crate) t: f64,
}

impl HitRecord {
    // Sets default values so that compiler is satisfied. SHOULD BE REWRITTEN if you want to use it
    // meaningfully.
    pub fn new() -> Self {
        HitRecord {
            point: Point::zero(),
            normal: Vec3::zero(),
            t: 0.0,
        }
    }
}
