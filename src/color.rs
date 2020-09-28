pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

/// Internally, color is stored as combination of 8-bit colors
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn from_fraction(r: f64, g: f64, b: f64) -> Option<Color> {
        if r > 1. || r < 0. || g > 1. || g < 0. || b > 1. || b < 0. {
            None
        } else {
            Some(Color {
                r: (r * 255.999) as u8,
                g: (g * 255.999) as u8,
                b: (b * 255.999) as u8,
            })
        }
    }

    /// Writes color of the pixel in format "R G B "
    /// Each value can have value from 0 to 255
    pub fn write(&self) {
        print!("{} {} {} ", self.r, self.g, self.b);
    }

    pub fn blue() -> Color {
        Color::from_fraction(0.5, 0.7, 1.0).unwrap()
    }

    pub fn white() -> Color {
        Color::from_fraction(1.0, 1.0, 1.0).unwrap()
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: (self * (rhs.r as f64)) as u8,
            g: (self * (rhs.g as f64)) as u8,
            b: (self * (rhs.b as f64)) as u8,
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