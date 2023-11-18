use std::ops::{Add, AddAssign, Rem, RemAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3d(f64, f64, f64);

impl Vec3d {
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    #[must_use]
    pub const fn x(&self) -> f64 {
        self.0
    }

    #[must_use]
    pub const fn y(&self) -> f64 {
        self.1
    }

    #[must_use]
    pub const fn z(&self) -> f64 {
        self.2
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
