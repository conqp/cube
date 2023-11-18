use crate::Vec3d;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::f64::consts::TAU;

#[derive(Debug)]
pub struct RandomRotation {
    position: Vec3d,
    rng: ThreadRng,
}

impl RandomRotation {
    #[must_use]
    pub fn new(position: Vec3d) -> Self {
        Self {
            position,
            rng: thread_rng(),
        }
    }
}

impl Default for RandomRotation {
    fn default() -> Self {
        Self::new(Vec3d::default())
    }
}

impl Iterator for RandomRotation {
    type Item = Vec3d;

    fn next(&mut self) -> Option<Self::Item> {
        self.position.0 += self.rng.gen::<f64>() / 10.0;
        self.position.0 %= TAU;
        self.position.1 += self.rng.gen::<f64>() / 10.0;
        self.position.1 %= TAU;
        self.position.2 += self.rng.gen::<f64>() / 10.0;
        self.position.2 %= TAU;
        Some(self.position)
    }
}