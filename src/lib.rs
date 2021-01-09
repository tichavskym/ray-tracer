use image::{ImageBuffer, Rgb};
use rand::{thread_rng, Rng};

use camera::Sensor;
use color::Color;
use ray::Ray;
use std::f64::consts::PI;
use vec3::Vec3;
// For better readability, making its own module would make things complicated
use vec3::Vec3 as Point;

mod camera;
mod color;
mod ray;
mod vec3;

const INFINITY: f64 = f64::MAX;
const SAMPLES_PER_PIXEL: u16 = 50; // Antialiasing
const MAX_DEPTH: u16 = 10;

/// Holds information about the resulting image.
struct Image {
    aspect_ratio: f64,
    width: u32,
    height: u32,
}

impl Image {
    fn new(width: u32, aspect_ratio: f64) -> Image {
        let aspect_ratio = aspect_ratio;
        Image {
            aspect_ratio,
            width,
            height: (width as f64 / aspect_ratio) as u32,
        }
    }
}

// Stores information about intersection of ray and the object.
struct HitRecord {
    // Point of intersection.
    point: Point,
    // Normal surface vector at the point of intersection.
    normal: Vec3,
    // Parameter that says where on the ray the intersection happend.
    t: f64,
    // True means the ray hit the object from the outside, false from the inside.
    front_face: bool,
}

impl HitRecord {
    // Sets default values so that copiler is satisfied. SHOULD BE REWRITTEN if you wanna meaningfully use it.
    fn new() -> HitRecord {
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

// Every ray traced object should implement this trait
trait Hittable {
    // Return true if sphere and ray intersect; data about point closer to the camera are saved
    // into `HitRecord` struct.
    // Hit only counts if it's from interval (t_min, t_min).
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

trait Material {
    // Return reflected ray, all necessary info is stored in `rec`
    fn scatter(&self, rec: &HitRecord) -> Ray;
    // Return absorption of the material as fraction
    fn absorption(&self) -> Color;
}

// Only one trait can be passed as an argument
trait TraceableObjects: Hittable + Material {}

// Describes a material that is used to model diffused object surfaces
struct Lambertian {
    // How much light is reflected (as fraction)
    absorption: Color,
}

impl Lambertian {
    fn new(absorption: Color) -> Lambertian {
        Lambertian { absorption }
    }
}

impl Material for Lambertian {
    fn scatter(&self, rec: &HitRecord) -> Ray {
        // Random unit vector je vicemene chovani toho materialu
        let mut direction = &rec.normal + &(Vec3::random_unit_vector());

        // If one of the components was zero, it would cause computational problems
        if direction.near_zero() {
            direction = rec.normal;
        }

        let new_ray = Ray::new(rec.point, direction);
        return new_ray;
    }

    fn absorption(&self) -> Color {
        return self.absorption.copy();
    }
}

// Represents ray traced object: sphere
struct Sphere {
    center: Point,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    fn new(center: Point, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Material for Sphere {
    fn scatter(&self, rec: &HitRecord) -> Ray {
        self.material.scatter(&rec)
    }

    fn absorption(&self) -> Color {
        return self.material.absorption();
    }
}

impl Hittable for Sphere {
    // Return true if sphere and ray intersect; data about point closer to the camera are saved
    // into `HitRecord` struct.
    //
    // There is a quadratic equation that describes spacial geometry containing ray and sphere.
    // If the equation has one (discriminant = 0) or two roots (discr > 0), the ray touches or
    // intersects the sphere, respectively. The exact equation and
    // more thorough explanation can be found at:
    // https://raytracing.github.io/books/RayTracingInOneWeekend.html#addingasphere
    //
    // We're not interested in one root, bcs the ray would most likely reflect somewhere else.
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

        return true;
    }
}

impl TraceableObjects for Sphere {}

fn set_scene_objects(objects: &mut Vec<Box<dyn TraceableObjects>>) {
    let diffused = Box::new(Lambertian::new(Color::from_frac(0.7, 0.7, 0.3).unwrap()));
    let sphere = Sphere::new(Point::new(0., 0., -1.), 0.5, diffused);
    objects.push(Box::new(sphere));
    let diffused = Box::new(Lambertian::new(Color::from_frac(0.9, 0.6, 0.1).unwrap()));
    let sphere = Sphere::new(Point::new(0., -100.5, -1.), 100., diffused);
    objects.push(Box::new(sphere));
}

pub fn run() {
    let image = Image::new(400, 16.0 / 9.0);
    let camera_viewport = Sensor::new(2.0, image.aspect_ratio, 1.0);

    let mut scene_objects: Vec<Box<dyn TraceableObjects>> = Vec::new();
    set_scene_objects(&mut scene_objects);

    let image_buffer = calculate_image(&camera_viewport, &image, &scene_objects);
    save_image(&image_buffer);
}

// Iterate over every pixel in the image, use two offset vectors `u` and `v` to convert
// the image pixel location to a fraction from 0 to 1, so that we can use them when
// creating the ray which gets calculated from the virtual viewport location
// of a pixel (virtual viewport is later scaled and saved as the image).
// The color of each pixel is computed multiple times (SAMPLES_PER_PIXEL times, every time with
// random deviation) so that we get "white noise" and aliased picture is created. All of the colors
// are added and then mathematically transformed into final color of the pixel.
fn calculate_image(
    cam: &Sensor,
    image: &Image,
    scene_objects: &Vec<Box<dyn TraceableObjects>>,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image_buffer = image::ImageBuffer::new(image.width, image.height);
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        // Used for color sampling of the pixel.
        let mut color = Color::black();
        for _ in 0..SAMPLES_PER_PIXEL {
            let u: f64 = (x as f64 + random_double()) / (image.width as f64 - 1.0);
            let v: f64 = (image.height as f64 - 1. - y as f64 + random_double())
                / (image.height as f64 - 1.0);

            let ray = cam.calculate_ray(u, v);
            let sample_color = calculate_color(ray, &scene_objects, MAX_DEPTH);
            color.add_sample(sample_color);
        }
        color.combine_samples(SAMPLES_PER_PIXEL);
        // Rgb is struct holding array of three elements
        *pixel = image::Rgb(color.get_u8());
    }
    image_buffer
}

// Returns random number in range from 0.0 (included) to 1.0 (excluded)
fn random_double() -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(0.0..1.0)
}

fn save_image(image_buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) {
    image_buffer.save("image.png").unwrap();
}

// If ray hits the sphere, return color based on the surface normal vector of the collision
// point on sphere or background
fn calculate_color(ray: Ray, shapes: &Vec<Box<dyn TraceableObjects>>, depth: u16) -> Color {
    if depth <= 0 {
        return Color::black();
    }

    let mut rec: HitRecord = HitRecord::new();
    for s in shapes {
        // https://raytracing.github.io/books/RayTracingInOneWeekend.html#diffusematerials/
        if s.hit(&ray, 0.001, INFINITY, &mut rec) {
            let new_ray = s.scatter(&rec);
            return s.absorption() * calculate_color(new_ray, shapes, depth - 1);
        }
    }
    generate_background_color(ray)
}

// Linearly blend white and blue depending on the y coordinate. Because we normalize the vector
// (we transform it to the unit vector), it changes shade even based on the x coordinate (as we
// change value of y, the value of x has to change too).
fn generate_background_color(r: Ray) -> Color {
    let unit_direction: Vec3 = r.unit_vector();

    // from ( -1.0 ... 1.0) to ( 0.0 ... 1.0)
    let t = 0.5 * (unit_direction.y() + 1.0);

    // blendedValue = ( 1.0 - t ) startValue + t * endValue
    (1.0 - t) * Color::white() + t * Color::blue()
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
