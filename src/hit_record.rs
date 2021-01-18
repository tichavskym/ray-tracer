use crate::ray::Ray;
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
    // True means the ray hit the object from the outside, false from the inside.
    front_face: bool,
}

impl HitRecord {
    // Sets default values so that compiler is satisfied. SHOULD BE REWRITTEN if you want to use it
    // meaningfully.
    pub fn new() -> Self {
        HitRecord {
            point: Point::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: true,
        }
    }

    // Set `front_face` and if it's false (ray is inside of the object), it makes
    // the normal vector negative, so that it in the direction agains the ray.
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) > 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
