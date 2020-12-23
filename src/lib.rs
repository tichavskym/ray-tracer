use crate::color::Color;
use image::{ImageBuffer, Rgb};
use vec3::Vec3;
use std::f64::consts::PI as PI;

mod color;
mod vec3;

const INFINITY: f64 = f64::MAX;

/// Image sensor (imager) parameters:
/// `Focal length` is a distance between projection plane to projection point (origin).
/// `origin` and `lower_left_corner` together with `focal_length` give us spacial orientation of
/// the virtual sensor.
struct Sensor {
    height: f64,
    width: f64,
    focal_length: f64,
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Sensor {
    fn new(height: f64, aspect_ratio: f64, focal_length: f64) -> Sensor {
        let origin = Vec3::zero();
        let width = aspect_ratio * height;
        let horizontal = Vec3::new(width, 0., 0.);
        let vertical = Vec3::new(0., height, 0.);

        Sensor {
            height,
            width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner: &origin
                - &(&horizontal / 2.0)
                - &vertical / 2.0
                - Vec3::new(0., 0., focal_length),
        }
    }

    fn lower_left_corner(&self) -> &Vec3 {
        &self.lower_left_corner
    }

    fn horizontal(&self) -> &Vec3 {
        &self.horizontal
    }

    fn vertical(&self) -> &Vec3 {
        &self.vertical
    }

    fn origin(&self) -> &Vec3 {
        &self.origin
    }
}

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

/// Ray is a function in a form: `P(t) = A + tb`, where A is an origin, t is a parameter and
/// b is a direction
#[derive(Debug)]
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    /// Get value of point `P(t) = A + direction * t`
    fn at(&self, t: f64) -> Vec3 {
        &self.origin + &(t * &self.direction)
    }

    fn unit_vector(&self) -> Vec3 {
        &self.direction / self.direction.length()
    }

    fn direction(&self) -> Vec3 {
        self.direction.clone()
    }

    fn origin(&self) -> Vec3 {
        self.origin.clone()
    }
}

// Stores information about intersection of ray and the object.
struct HitRecord {
    // Point of intersection.
    point: Vec3,
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
            point: Vec3::zero(),
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

trait Hittable {
    /// Hit only counts if it's from interval (t_min, t_min).
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

// Represents ray traced object: sphere
struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    // Return true if sphere and ray intersect, data about point closer to the camera are saved
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

fn set_scene_objects(objects: &mut Vec<Box<dyn Hittable>>) {
    let sphere = Sphere::new(Vec3::new(0., 0., -1.), 0.5);
    objects.push(Box::new(sphere));
    let sphere = Sphere::new(Vec3::new(0., -100.5, -1.), 100.);
    objects.push(Box::new(sphere));
}

pub fn run() {
    let image = Image::new(400, 16.0 / 9.0);
    let camera_viewport = Sensor::new(2.0, image.aspect_ratio, 1.0);

    let mut scene_objects: Vec<Box<dyn Hittable>> = Vec::new();
    set_scene_objects(&mut scene_objects);

    let image_buffer = calculate_image(&camera_viewport, &image, &scene_objects);
    save_image(&image_buffer);
}

// Iterate over every pixel in the image, use two offset vectors `u` and `v` to convert
// the image pixel location to a fraction from 0 to 1, so that we can use them when
// creating the ray which gets calculated from the virtual viewport location
// of a pixel (virtual viewport is later scaled and saved as the image).
fn calculate_image(camera_viewport: &Sensor, image: &Image, scene_objects: &Vec<Box<dyn Hittable>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image_buffer = image::ImageBuffer::new(image.width, image.height);
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let u: f64 = x as f64 / (image.width as f64 - 1.0);
        let v: f64 = (image.height as f64 - 1. - y as f64) / (image.height as f64 - 1.0);

        let ray = calculate_ray(u, v, &camera_viewport);
        let color = calculate_color(ray, &scene_objects);
        // Rgb is struct holding array of three elements
        *pixel = image::Rgb([color.r(), color.g(), color.b()]);
    }
    image_buffer
}

fn save_image(image_buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) {
    image_buffer.save("image.png").unwrap();
}

// The ray goes from origin to the pixel in the virtual viewport which is specified by offset
// vectors `u` and `v`.
fn calculate_ray(u: f64, v: f64, camera_viewport: &Sensor) -> Ray {
    Ray::new(
        camera_viewport.origin,
        &(&camera_viewport.lower_left_corner + &(u as f64 * &camera_viewport.horizontal))
            + &(v as f64 * &camera_viewport.vertical)
            - camera_viewport.origin,
    )
}

// If ray hits the sphere, return color based on the surface normal vector of the collision
// point on sphere or background
fn calculate_color(ray: Ray, shapes: &Vec<Box<dyn Hittable>>) -> Color {
    let mut rec: HitRecord = HitRecord::new();
    for s in shapes {
        if s.hit(&ray, 0., 2000., &mut rec) {
            let n = rec.normal;
            // Transformation from ( -1.0 ... 1.0) to ( 0.0 ... 1.0)
            return Color::from_fraction((n.x() + 1.) / 2., (n.y() + 1.) / 2., (n.z() + 1.) / 2.)
            .unwrap();
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
