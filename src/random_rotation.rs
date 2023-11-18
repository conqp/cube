use crate::Vec3d;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::f64::consts::TAU;

const DEFAULT_OFFSET: f64 = 0.1;
const DEFAULT_SCALING: f64 = 0.1;

#[derive(Debug)]
pub struct RandomRotation {
    position: Vec3d,
    rng: ThreadRng,
    offset: f64,
    scaling: f64,
}

impl RandomRotation {
    #[must_use]
    pub fn new(position: Vec3d, offset: f64, scaling: f64) -> Self {
        Self {
            position,
            rng: thread_rng(),
            offset,
            scaling,
        }
    }

    fn random(&mut self) -> f64 {
        (self.rng.gen::<f64>() - self.offset) * self.scaling
    }
}

impl Default for RandomRotation {
    fn default() -> Self {
        Self::new(Vec3d::default(), DEFAULT_OFFSET, DEFAULT_SCALING)
    }
}

impl Iterator for RandomRotation {
    type Item = Vec3d;

    fn next(&mut self) -> Option<Self::Item> {
        let rand = Vec3d::new(self.random(), self.random(), self.random());
        self.position += rand;
        self.position %= TAU;
        Some(self.position)
    }
}
