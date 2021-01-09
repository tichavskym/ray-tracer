use rand::{thread_rng, Rng};

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
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn x(&self) -> f64 {
        return self.x;
    }

    pub fn y(&self) -> f64 {
        return self.y;
    }

    pub fn z(&self) -> f64 {
        return self.z;
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }

    // Lambertian reflection, drop in replacement for `random_in_unit_sphere`,
    // with distribution of cos x
    pub fn random_unit_vector() -> Vec3 {
        unit_vector(random_in_unit_sphere())
    }

    // Return true if any of the vector components is near zero
    pub fn near_zero(&self) -> bool {
        let eps = 0.0000001;
        return (self.x.abs() < eps) && (self.y.abs() < eps) && (self.z.abs() < eps);
    }
}

// Returns random vector, distribution cos^3 x
fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random(-1., 1.);
        if p.length_squared() < 1. {
            return p;
        }
    }
}

fn unit_vector(vec: Vec3) -> Vec3 {
    vec / vec.length()
}

fn random(min: f64, max: f64) -> Vec3 {
    let mut rng = thread_rng();
    return Vec3 {
        x: rng.gen_range(min..max),
        y: rng.gen_range(min..max),
        z: rng.gen_range(min..max),
    };
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

// Make vector negative
impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
