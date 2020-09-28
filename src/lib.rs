use crate::color::Color;
use vec3::Vec3;

mod color;
mod vec3;

pub fn run () {
    // Image parameters
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;

    // Camera
    let sensor_height = 2.0;
    let sensor_width = ASPECT_RATIO * sensor_height;
    // focal length = distance between projection plane to projection point (origin)
    let focal_length = 1.0;

    // origin = camera = eye
    let origin = Vec3::zero();
    let horizontal = Vec3::new(sensor_width, 0., 0.);
    let vertical = Vec3::new(0., sensor_height, 0.);
    let lower_left_corner = &origin - &(&horizontal / 2.0) - &vertical / 2.0 - Vec3::new(0., 0., focal_length);

    // The image format we're using is ppm. More info: https://en.wikipedia.org/wiki/Netpbm#PPM_example
    // Print header of the PPM image
    const MAX_COLOR: u16 = 256;
    println!("P3\n{} {}\n{}\n", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR);

    // Print individual pixels; progress info goes into standard error so that we can redirect
    // standard output to the .ppm file.
    for h in 0..IMAGE_HEIGHT {
        // eprintln!("Remaining lines: {}", IMAGE_HEIGHT - h);
        for w in 0..IMAGE_WIDTH {
            let u = w as f64/ (IMAGE_WIDTH as f64 - 1.0);
            let v = h as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let r: Ray = Ray::new(
                origin,
                &(&lower_left_corner + &(u as f64 * &horizontal)) + &(v as f64 * &vertical) - origin,
            );
            let pixel_color: Color = ray_color_background(r);
            pixel_color.write();
        }
        println!();
    }
    eprintln!("Done");
}

/// Ray is a function in a form: `P(t) = A + tb`, where A is an origin, t is a parameter and b is a direction
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
}

/// Linearly blend white and blue depending on the y coordinate. Because we normalize the vector
/// (we transform it to the unit vector), it changes shade even based on the x coordinate (as we
/// change value of y, the value of x has to change too).
fn ray_color_background(r: Ray) -> Color {
    let unit_direction: Vec3 = r.unit_vector();

    // from ( -1.0 ... 1.0) to ( 0.0 ... 1.0)
    let t = 0.5 * (unit_direction.y() + 1.0);

    // blendedValue = ( 1.0 - t ) startValue + t * endValue
    (1.0 - t) * Color::blue() + t * Color::white()
}