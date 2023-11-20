use std::ops::{Add, AddAssign, Rem, RemAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3d(f64, f64, f64);

impl Vec3d {
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
    #[must_use]
    pub fn angle(self, other: Self) -> Self {
        Self(
            self.x_angle(&other),
            self.y_angle(&other),
            self.z_angle(&other),
        )
    }

    fn x_angle(&self, other: &Self) -> f64 {
        (other.0 * self.1.cos()).mul_add(
            self.2.cos(),
            (other.2 * self.0.sin()).mul_add(
                self.2.sin(),
                (other.1 * self.0.cos()).mul_add(
                    self.2.sin(),
                    (other.1 * self.0.sin() * self.1.sin()).mul_add(
                        self.2.cos(),
                        -other.2 * self.0.cos() * self.1.sin() * self.2.cos(),
                    ),
                ),
            ),
        )
    }

    fn y_angle(&self, other: &Self) -> f64 {
        (other.0 * self.1.cos()).mul_add(
            -self.2.sin(),
            (other.2 * self.0.cos() * self.1.sin()).mul_add(
                self.2.sin(),
                (other.1 * self.0.sin() * self.1.sin()).mul_add(
                    -self.2.sin(),
                    (other.1 * self.0.cos())
                        .mul_add(self.2.cos(), other.2 * self.0.sin() * self.2.cos()),
                ),
            ),
        )
    }

    fn z_angle(&self, other: &Self) -> f64 {
        other.0.mul_add(
            self.1.sin(),
            (other.2 * self.0.cos()).mul_add(self.1.cos(), -other.1 * self.0.sin() * self.1.cos()),
        )
    }
}

impl Add for Vec3d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl From<Vec3d> for (f64, f64, f64) {
    fn from(vec3d: Vec3d) -> Self {
        (vec3d.0, vec3d.1, vec3d.2)
    }
}

impl From<(f64, f64, f64)> for Vec3d {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self(x, y, z)
    }
}

impl Rem<f64> for Vec3d {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self::Output {
        Self(self.0 % rhs, self.1 % rhs, self.2 % rhs)
    }
}

impl RemAssign<f64> for Vec3d {
    fn rem_assign(&mut self, rhs: f64) {
        self.0 %= rhs;
        self.1 %= rhs;
        self.2 %= rhs;
    }
}
