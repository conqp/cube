use std::ops::{Add, AddAssign, Rem, RemAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3d(f64, f64, f64);

impl Vec3d {
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    #[must_use]
    pub fn x_angle(&self, i: f64, j: f64, k: f64) -> f64 {
        (i * self.1.cos()).mul_add(
            self.2.cos(),
            (k * self.0.sin()).mul_add(
                self.2.sin(),
                (j * self.0.cos()).mul_add(
                    self.2.sin(),
                    (j * self.0.sin() * self.1.sin()).mul_add(
                        self.2.cos(),
                        -k * self.0.cos() * self.1.sin() * self.2.cos(),
                    ),
                ),
            ),
        )
    }

    #[must_use]
    pub fn y_angle(&self, i: f64, j: f64, k: f64) -> f64 {
        (i * self.1.cos()).mul_add(
            -self.2.sin(),
            (k * self.0.cos() * self.1.sin()).mul_add(
                self.2.sin(),
                (j * self.0.sin() * self.1.sin()).mul_add(
                    -self.2.sin(),
                    (j * self.0.cos()).mul_add(self.2.cos(), k * self.0.sin() * self.2.cos()),
                ),
            ),
        )
    }

    #[must_use]
    pub fn z_angle(&self, i: f64, j: f64, k: f64) -> f64 {
        i.mul_add(
            self.1.sin(),
            (k * self.0.cos()).mul_add(self.1.cos(), -j * self.0.sin() * self.1.cos()),
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
