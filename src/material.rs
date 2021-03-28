use std::marker::{Send, Sync};

use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material: Send + Sync {
    /// Returns reflected ray and stores all necessary info about intersection into `rec`.
    fn scatter(&self, rec: &HitRecord, ray_in: &Ray) -> Option<Ray>;
    /// Returns color of the material
    fn attenuation(&self) -> Color;
}

/// Describes a material that is used to model diffused object surfaces
pub struct Lambertian {
    /// How much light is reflected from the surface (as fraction)
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, rec: &HitRecord, _ray_in: &Ray) -> Option<Ray> {
        // Random unit vector is a behaviour of the material
        let mut direction = rec.normal + Vec3::random_unit_vector();

        if direction.near_zero() {
            direction = rec.normal;
        }

        let new_ray = Ray::new(rec.point, direction);
        Some(new_ray)
    }

    fn attenuation(&self) -> Color {
        self.albedo.copy()
    }
}

/// There are two `Metal` materials, one of them is shiny and the other is fuzzy.
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn fuzzy(albedo: Color, fuzz: f64) -> Metal {
        let fuzz = if fuzz < 1. { fuzz } else { 1. };
        Metal { albedo, fuzz }
    }

    pub fn shiny(albedo: Color) -> Metal {
        Metal { albedo, fuzz: 0. }
    }
}

/// Reflects vector `v` from surface given by `normal` vector
fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    let b = Vec3::dot(v, normal);
    v - 2. * b * &normal
}

impl Material for Metal {
    fn scatter(&self, rec: &HitRecord, ray_in: &Ray) -> Option<Ray> {
        let reflected = reflect(ray_in.direction().unit_vector(), rec.normal);
        let scattered = Ray::new(
            rec.point,
            reflected + self.fuzz * &Vec3::random_unit_vector(),
        );
        if Vec3::dot(scattered.direction(), rec.normal) > 0. {
            Some(scattered)
        } else {
            None
        }
    }

    fn attenuation(&self) -> Color {
        self.albedo.copy()
    }
}
