// This struct will be used for 3D Points, Directions, ...
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn length_squared(&self) -> f64 {
        &self.x * &self.x + &self.y * &self.y + &self.z * &self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn zero() -> Vec3 {
        Vec3{
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
    pub fn y(&self) -> f64 {
        return self.y
    }
}

/// Addition of two `&Vec3` structs. Implemented as adding each of the coordinates together.
impl std::ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: &self.x + rhs.x,
            y: &self.y + rhs.y,
            z: &self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: &self.x - &rhs.x,
            y: &self.y - &rhs.y,
            z: &self.z - &rhs.z,
        }
    }
}

impl std::ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: &self.x - &rhs.x,
            y: &self.y - &rhs.y,
            z: &self.z - &rhs.z,
        }
    }
}

/// The `std::ops::Mul` trait is used to specify the functionality of `*`.
/// The following block implements the operation: f64 * &Vec3.
/// `rhs` stands for right hand side
impl std::ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: &self * rhs.x,
            y: &self * rhs.y,
            z: &self * rhs.z,
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: &self.x / rhs,
            y: &self.y / rhs,
            z: &self.z / rhs,
        }
    }
}

impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: &self.x / rhs,
            y: &self.y / rhs,
            z: &self.z / rhs,
        }
    }
}