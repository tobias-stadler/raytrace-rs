use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::{Rng, RngCore};

#[allow(non_camel_case_types)]
pub type fVec = f32;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    pub x: fVec,
    pub y: fVec,
    pub z: fVec,
}

impl Vec3 {
    #[inline]
    pub fn new(x: fVec, y: fVec, z: fVec) -> Self {
        Self { x: x, y: y, z: z }
    }

    #[inline]
    pub fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline]
    pub fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline]
    pub fn unit_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    #[inline]
    pub fn unit_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    #[inline]
    pub fn random(rng: &mut (impl RngCore + ?Sized), min: fVec, max: fVec) -> Self {
        Self {
            x: rng.gen_range(min..=max),
            y: rng.gen_range(min..=max),
            z: rng.gen_range(min..=max),
        }
    }

    #[inline]
    pub fn is_tiny(&self, tolerance: fVec) -> bool {
        self.x.abs() < tolerance && self.y.abs() < tolerance && self.z.abs() < tolerance
    }

    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline]
    pub fn reflect(self, normal: Self) -> Self {
        self + normal * ((self * normal) * -2.0)
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn refract(self, normal: Self, ior: fVec, rng: &mut (impl RngCore + ?Sized)) -> Self {
        let I = self.unit();
        let IdotN = I * normal;

        let c_ior;
        let c_normal;

        let cos_theta = if IdotN < 0.0 {
            c_ior = 1.0 / ior;
            c_normal = normal;
            -IdotN
        } else {
            c_ior = ior;
            c_normal = -normal;
            IdotN
        };

        let sin_theta_squared = 1.0 - cos_theta * cos_theta;
        let sin_theta = sin_theta_squared.abs().sqrt();

        if c_ior * sin_theta > 1.0 {
            return I.reflect(c_normal);
        }

        let rand = rng.gen_range(0.0..1.0);
        if rand < reflectance(cos_theta, ior) {
            return I.reflect(c_normal);
        }

        let A = (I + c_normal * cos_theta) * c_ior;
        let B = c_normal * (-(1.0 - c_ior * c_ior * sin_theta_squared).abs().sqrt());

        A + B
    }

    #[inline]
    pub fn length(self) -> fVec {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    pub fn unit(self) -> Self {
        let len = self.length();
        self / len
    }
}

fn reflectance(cos: fVec, ior: fVec) -> fVec {
    let r0 = (1.0 - ior) / (1.0 + ior);
    let r0_squared = r0 * r0;
    r0_squared + (1.0 - r0_squared) * ((1.0 - cos).powi(5))
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<fVec> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: fVec) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Mul for Vec3 {
    type Output = fVec;

    #[inline]
    fn mul(self, other: Self) -> fVec {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<fVec> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, other: fVec) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<fVec> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, other: fVec) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<fVec> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: fVec) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}
