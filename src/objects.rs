use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::Vec3 as Point;
use crate::{Hittable, TraceableObjects};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Material for Sphere {
    fn scatter(&self, rec: &HitRecord, ray_in: &Ray) -> Option<Ray> {
        self.material.scatter(&rec, ray_in)
    }

    fn attenuation(&self) -> Color {
        self.material.attenuation()
    }
}

impl Hittable for Sphere {
    /// The intersection is a solution of a quadratic equation describing a spacial geometry of ray
    /// and sphere. If it has one root, the ray only touches the sphere (ignored for simplicity
    /// reasons), if it has two roots, it intersects the sphere, therefore `true` is returned and
    /// `HitRecord` is set.
    ///
    /// More thorough explanation can be found at:
    /// https://raytracing.github.io/books/RayTracingInOneWeekend.html#addingasphere
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // oc = line segment between origin and center
        let oc = ray.origin() - self.center;
        let a = Vec3::dot(ray.direction(), ray.direction());
        let b = 2.0 * Vec3::dot(ray.direction(), oc);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - 4. * a * c;
        if discriminant < 0.0 {
            return false;
        }

        let mut root = (-b - discriminant.sqrt()) / (2.0 * a);
        if root < t_min || t_max < root {
            root = (-b + discriminant.sqrt()) / (2.0 * a);
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        rec.normal = (rec.point - self.center) / self.radius;

        true
    }
}

impl TraceableObjects for Sphere {}
