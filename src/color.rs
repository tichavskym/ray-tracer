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

    /// Returns `None` if any of the arguments is larger than 1
    pub fn from_frac(r: f64, g: f64, b: f64) -> Option<Color> {
        if r > 1. || r < 0. || g > 1. || g < 0. || b > 1. || b < 0. {
            None
        } else {
            Some(Color { r, g, b })
        }
    }

    /// All fields are set to value 0.0
    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    /// Adds colors without checking overflow. Therefore, the resulting color does not have to make
    /// sense.
    pub fn add_sample(&mut self, color: Color) {
        self.r += color.r;
        self.g += color.g;
        self.b += color.b;
    }

    /// Combines samples to get final color of the pixel using "white noise" method.
    ///
    /// `Color` on which the method is called, is expected to be sum of samples (how many of them is
    /// given by parameter `samples`).
    pub fn combine_samples(&mut self, samples: u16) {
        // Scale and gamma-correct for gamma=2.0 (sqrt).
        let scale = 1.0 / samples as f64;
        self.r = (self.r * scale).sqrt();
        self.g = (self.g * scale).sqrt();
        self.b = (self.b * scale).sqrt();

        // Transform each component to [0,255] range
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

    pub fn copy(&self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

/// Clamp value x to the range [min, max]
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

impl std::ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color {
            r: (self.r * rhs.r),
            g: (self.g * rhs.g),
            b: (self.b * rhs.b),
        }
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
