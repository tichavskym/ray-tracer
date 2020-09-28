use crate::color::Color;
use vec3::Vec3;

mod color;
mod vec3;

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
            lower_left_corner: &origin - &(&horizontal / 2.0) - &vertical / 2.0 - Vec3::new(0., 0., focal_length),
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
    height: u32
}
impl Image {
    fn new(width: u32, aspect_ratio: f64) -> Image {
        let aspect_ratio = aspect_ratio;
        Image {
            aspect_ratio,
            width,
            height: (width as f64 / aspect_ratio) as u32
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

// Represents ray traced object: sphere
struct Sphere {
    center: Vec3,
    radius: f64
}
impl Sphere {
    fn new(center: Vec3, radius: f64) -> Sphere{
        Sphere { center, radius }
    }
}

pub fn run() {
    // There are values hardcoded into this program, so if you want to change any of these values (image
    // or camera_viewport fields), go through the calculations to make sure it will render the image correctly.
    let image = Image::new(400, 16.0 / 9.0);
    let camera_viewport = Sensor::new( 2.0, image.aspect_ratio, 1.0);

    // The image format we're using is ppm. More info: https://en.wikipedia.org/wiki/Netpbm#PPM_example
    // Print header of the PPM image
    const MAX_COLOR: u16 = 256;
    println!("P3\n{} {}\n{}\n", image.width, image.height, MAX_COLOR);

    // Transversing the image pixels from the lower left hand corner, and using two offset vectors
    // `u` and `v` to convert the image pixel location to a fraction from 0 to 1, so that we can use
    // them when creating the ray which gets calculated from the virtual viewport location
    // of a pixel (virtual viewport is later scaled and saved as the image).
    for h in 0..image.height {
        eprintln!("Remaining lines: {}", image.height - h);
        for w in 0..image.width {
            let u: f64 = w as f64 / (image.width as f64 - 1.0);
            let v : f64= h as f64 / (image.height as f64 - 1.0);

            let ray = calculate_the_ray(u, v, &camera_viewport);
            let color = calculate_color_for_this_point(ray);
            color.write();
        }
        // PPM formatting -> each line of pixels ends with new line character
        println!();
    }
    eprintln!("Done");
}

// The ray goes from origin to the pixel in the virtual viewport which is specified by offset
// vectors `u` and `v`.
fn calculate_the_ray(u: f64, v: f64, camera_viewport: &Sensor) -> Ray {
    Ray::new(
        camera_viewport.origin,
        &(&camera_viewport.lower_left_corner + &(u as f64 * &camera_viewport.horizontal))
            + &(v as f64 * &camera_viewport.vertical) - camera_viewport.origin,
    )
}

/// If ray hits the sphere, return red color, else return background color
fn calculate_color_for_this_point(ray: Ray) -> Color {
    if hit_sphere(
        &ray,
        Sphere::new(Vec3::new(0., 0., -1.), 0.5)
    ) {
        Color::from_fraction(1., 0., 0.).unwrap()
    } else {
        generate_background_color(ray)
    }
}

// There is quadratic equation that describes spacial geometry containing ray and sphere.
// If the equation has one or two roots, the ray touches or intersects the sphere, respectively.
// Equation has one root if discriminant = 0, two if discriminant > 0. The exact equation and
// more thorough explanation can be found at:
// https://raytracing.github.io/books/RayTracingInOneWeekend.html#addingasphere
// We're not interested in discriminant = 0, bcs the ray would most likely reflect somewhere else
// afaik. I just trust the author of this book that we should ignore it.
fn hit_sphere(ray: &Ray, sphere: Sphere) -> bool {
    // oc = line segment between origin and center
    let oc = ray.origin() - sphere.center;
    let a = Vec3::dot(ray.direction(), ray.direction());
    let b = 2.0 * Vec3::dot(ray.direction(), oc );
    let c = Vec3::dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4. * a * c;
    discriminant > 0.0
}

/// Linearly blend white and blue depending on the y coordinate. Because we normalize the vector
/// (we transform it to the unit vector), it changes shade even based on the x coordinate (as we
/// change value of y, the value of x has to change too).
fn generate_background_color(r: Ray) -> Color {
    let unit_direction: Vec3 = r.unit_vector();

    // from ( -1.0 ... 1.0) to ( 0.0 ... 1.0)
    let t = 0.5 * (unit_direction.y() + 1.0);

    // blendedValue = ( 1.0 - t ) startValue + t * endValue
    (1.0 - t) * Color::blue() + t * Color::white()
}