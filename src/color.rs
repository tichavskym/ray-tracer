pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn from_u8(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: (r as f64) / 255.,
            g: (g as f64) / 255.,
            b: (b as f64) / 255.,
        }
    }

    // Returns None if any of the arguments is larger han 1
    pub fn from_frac(r: f64, g: f64, b: f64) -> Option<Color> {
        if r > 1. || r < 0. || g > 1. || g < 0. || b > 1. || b < 0. {
            None
        } else {
            Some(Color { r, g, b })
        }
    }

    // All fields are set to value 0.0
    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    // This method is used for antialiasing of individual pixel together with `black()` and
    // `combine_samples()` methods. It adds tuple components to corresponding components of `Color`,
    // without checking for overflow. You can therefore end up with Color fields that don't make
    // sense.
    pub fn add_sample(&mut self, color: Color) {
        self.r += color.r;
        self.g += color.g;
        self.b += color.b;
    }

    // Makes the calculation of "white noise" antialiasing of the pixel. `Color`, on which
    // the method is called, is expected to be sum of samples, and their number is parameter of
    // this method.
    pub fn combine_samples(&mut self, samples_per_pixel: u16) {
        // Divide the color by the number of samples (scale) and gamma-correct for gamma=2.0 (sqrt).
        let scale = 1.0 / samples_per_pixel as f64;
        self.r = (self.r * scale).sqrt();
        self.g = (self.g * scale).sqrt();
        self.b = (self.b * scale).sqrt();

        // Write the translated [0,255] value of each color component.
        self.r = 256.0 * clamp(self.r, 0.0, 0.999);
        self.g = 256.0 * clamp(self.g, 0.0, 0.999);
        self.b = 256.0 * clamp(self.b, 0.0, 0.999);
    }

    pub fn get_u8(self) -> [u8; 3] {
        [self.r as u8, self.g as u8, self.b as u8]
    }

    pub fn blue() -> Color {
        Color::from_frac(0.5, 0.7, 1.0).unwrap()
    }

    pub fn white() -> Color {
        Color::from_frac(1.0, 1.0, 1.0).unwrap()
    }
}

// Clamp value x to the range [min, max]
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: (self * rhs.r),
            g: (self * rhs.g),
            b: (self * rhs.b),
        }
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Color {
        Color {
            r: &self.r + rhs.r,
            g: &self.g + rhs.g,
            b: &self.b + rhs.b,
        }
    }
}
