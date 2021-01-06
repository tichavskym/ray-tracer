pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
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
    pub fn add_sample(&mut self, tup: (f64, f64, f64)) {
        let (r, g, b) = tup;
        self.r += r;
        self.g += g;
        self.b += b;
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
