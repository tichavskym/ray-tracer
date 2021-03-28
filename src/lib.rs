use image::{ImageBuffer, Rgb};
use rand::{thread_rng, Rng};
use std::sync::{mpsc, Arc};

use camera::Sensor;
use color::Color;
use hit_record::HitRecord;
use material::{Lambertian, Material, Metal};
use objects::Sphere;
use ray::Ray;
use std::ops::Deref;
use thread_pool::ThreadPool;
use vec3::Vec3;
use vec3::Vec3 as Point; // For better understanding of the code

mod camera;
mod color;
mod hit_record;
mod material;
mod objects;
mod ray;
mod thread_pool;
mod vec3;

const INFINITY: f64 = f64::MAX;

/// Supersampling anti-aliasing parameter
const SAMPLES_PER_PIXEL: u16 = 16;
/// Upper limit for ray reflections
const MAX_DEPTH: u16 = 10;
const THREAD_COUNT: u8 = 8;
const OUTPUT_FILE_NAME: &str = "image.png";

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_ASPECT_RATIO: f64 = 16.0 / 9.0;
const CAM_FOCAL_LENGTH: f64 = 1.0;
const CAM_HEIGHT: f64 = 2.0;

/// Holds information about dimensions of the resulting image.
struct Image {
    width: u32,
    height: u32,
}

impl Image {
    fn new(width: u32, aspect_ratio: f64) -> Image {
        Image {
            width,
            height: (width as f64 / aspect_ratio) as u32,
        }
    }
}

/// Trait implemented by every ray traced object
trait Hittable {
    /// Returns `true` if the object and ray intersect. Data about intersection point closer to the
    /// camera are saved into `HitRecord` struct. Intersection point is calculated only on interval
    /// (t_min, t_man).
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

trait TraceableObjects: Hittable + Material {}

fn set_scene_objects(objects: &mut Vec<Box<dyn TraceableObjects>>) {
    let diffused = Box::new(Lambertian::new(Color::from_frac(0.8, 0.2, 0.2).unwrap()));
    let sphere = Sphere::new(Point::new(0., 0., -1.), 0.5, diffused);
    objects.push(Box::new(sphere));
    let metal = Box::new(Metal::fuzzy(Color::from_frac(0.8, 0.8, 0.8).unwrap(), 0.3));
    let sphere = Sphere::new(Point::new(-1., 0., -1.0), 0.5, metal);
    objects.push(Box::new(sphere));
    let metal = Box::new(Metal::shiny(Color::from_frac(0.5, 0.6, 0.6).unwrap()));
    let sphere = Sphere::new(Point::new(1., 0., -1.0), 0.5, metal);
    objects.push(Box::new(sphere));
    let diffused = Box::new(Lambertian::new(Color::from_frac(0.05, 0.5, 0.05).unwrap()));
    let sphere = Sphere::new(Point::new(0., -100.5, -1.), 100., diffused);
    objects.push(Box::new(sphere));
}

pub fn run() {
    let image = Image::new(IMAGE_WIDTH, IMAGE_ASPECT_RATIO);
    let camera_viewport = Sensor::new(CAM_HEIGHT, IMAGE_ASPECT_RATIO, CAM_FOCAL_LENGTH);

    let mut scene_objects: Vec<Box<dyn TraceableObjects>> = Vec::new();
    set_scene_objects(&mut scene_objects);

    let image_buffer = calculate_image(camera_viewport, image, scene_objects);
    save_image(&image_buffer, OUTPUT_FILE_NAME);
}

/// Iterates over every pixel in the image, calculates its color and returns the resulting image.
/// The whole computation is done in parallel (`THREAD_COUNT` constant).
fn calculate_image(
    cam: Sensor,
    image: Image,
    scene_objects: Vec<Box<dyn TraceableObjects + 'static>>,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image_buffer = image::ImageBuffer::new(image.width, image.height);
    let pool = ThreadPool::new(THREAD_COUNT).unwrap();
    // Channel for transmitting results back to the main thread
    let (sender, receiver) = mpsc::channel();

    // Every thread needs to own this data
    let cam = Arc::new(cam);
    let image = Arc::new(image);
    let scene_objects = Arc::new(scene_objects);

    // `h` and `w` give us location of the pixel in the image
    for h in 0..image.height {
        let cam_clone = cam.clone();
        let image_clone = image.clone();
        let scene_objects_clone = scene_objects.clone();
        let sender_clone = sender.clone();

        pool.execute(move || {
            for w in 0..image_clone.width {
                let color = get_pixel_color(&cam_clone, &image_clone, &scene_objects_clone, h, w);
                let image_color = image::Rgb(color.get_u8());

                let tuple = (w, h, image_color);
                sender_clone.send(tuple).unwrap();
            }
            log::info!("Finished rendering of line {}", h);
        });
    }
    // The original value has to be dropped, so that the receiving for loop below ends after all
    // threads finish their work.
    std::mem::drop(sender);

    for incoming in receiver {
        let (w, h, image_color) = incoming;
        image_buffer.put_pixel(w, h, image_color);
    }

    image_buffer
}

/// Computes color of the pixel at coordinates `w` and `h`. Uses two offset vectors `u` and `v` to convert
/// the image pixel location to a fraction from 0 to 1 (used with virtual viewport for ray calculation).
///
/// Uses Supersampling anti-aliasing with random algorithm (stochastic sampling).
fn get_pixel_color(
    cam_clone: &Arc<Sensor>,
    image_clone: &Arc<Image>,
    scene_objects_clone: &Arc<Vec<Box<dyn TraceableObjects>>>,
    h: u32,
    w: u32,
) -> Color {
    let mut color = Color::black();
    for _ in 0..SAMPLES_PER_PIXEL {
        let u: f64 = (w as f64 + random_double()) / (image_clone.width as f64 - 1.0);
        let v: f64 = (image_clone.height as f64 - 1. - h as f64 + random_double())
            / (image_clone.height as f64 - 1.0);

        let ray = cam_clone.calculate_ray(u, v);
        let sample_color = calculate_color(ray, scene_objects_clone, MAX_DEPTH);
        color.add_sample(sample_color);
    }
    color.combine_samples(SAMPLES_PER_PIXEL);
    color
}

/// This returns color based on the surface normal vector at the collision point with an object (or
/// multiple collisions) or background color.
fn calculate_color(ray: Ray, shapes: &Arc<Vec<Box<dyn TraceableObjects>>>, depth: u16) -> Color {
    if depth == 0 {
        return Color::black();
    }

    let mut rec: HitRecord = HitRecord::new();
    for s in shapes.deref() {
        // https://raytracing.github.io/books/RayTracingInOneWeekend.html#diffusematerials/
        if s.hit(&ray, 0.001, INFINITY, &mut rec) {
            let new_ray = s.scatter(&rec, &ray);
            return if new_ray.is_some() {
                s.attenuation() * calculate_color(new_ray.unwrap(), shapes, depth - 1)
            } else {
                Color::black()
            };
        }
    }
    linearly_blend_colors(ray, Color::white(), Color::blue())
}

/// Returns linearly blended color depending on the ray coordinates.
fn linearly_blend_colors(r: Ray, start_value: Color, end_value: Color) -> Color {
    // Normalizing the vector => as value of y changes, the value of x has to change too =>
    // resulting color is dependent on both coordinates
    let unit_direction: Vec3 = r.unit_vector();

    // Transform from ( -1.0 ... 1.0) to ( 0.0 ... 1.0)
    let t = 0.5 * (unit_direction.y() + 1.0);

    // blended_value = ( 1.0 - t ) start_value + t * end_value
    (1.0 - t) * start_value + t * end_value
}

/// Returns random number in range from 0.0 (included) to 1.0 (excluded)
fn random_double() -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(0.0..1.0)
}

fn save_image(image_buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>, filename: &str) {
    image_buffer.save(filename).unwrap();
}
