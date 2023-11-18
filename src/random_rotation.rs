use crate::Vec3d;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

const DEFAULT_OFFSET: f64 = 0.1;
const DEFAULT_SCALING: f64 = 0.1;

#[derive(Debug)]
pub struct RandomRotation {
    rng: ThreadRng,
    offset: f64,
    scaling: f64,
}

impl RandomRotation {
    #[must_use]
    pub fn new(offset: f64, scaling: f64) -> Self {
        Self {
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
        Self::new(DEFAULT_OFFSET, DEFAULT_SCALING)
    }
}

impl Iterator for RandomRotation {
    type Item = Vec3d;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Vec3d::new(self.random(), self.random(), self.random()))
    }
}
