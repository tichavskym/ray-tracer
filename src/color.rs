pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

/// Internally, colors are stored in percents, written as RGB numbers
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    /// Writes color of the pixel in format "R G B "
    /// Each value can have value from 0 to 255
    pub fn write(&self) {
        print!("{} {} {} ", (&self.r * 255.999) as u16, (&self.g * 255.999) as u16, (&self.b * 255.999) as u16);
    }

    pub fn blue() -> Color {
        Color::new(0.5, 0.7, 1.0)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
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