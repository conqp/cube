use crate::{Vec3d, Viewport};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Animation<'a, T, const K1: u8>
where
    T: Iterator<Item = Vec3d>,
{
    animator: T,
    frequency: Duration,
    viewport: Viewport<'a, K1>,
}

impl<'a, T, const K1: u8> Animation<'a, T, K1>
where
    T: Iterator<Item = Vec3d>,
{
    #[must_use]
    pub const fn new(animator: T, frequency: Duration, viewport: Viewport<'a, K1>) -> Self {
        Self {
            animator,
            frequency,
            viewport,
        }
    }

    pub fn run(&mut self) {
        for rotation in &mut self.animator {
            self.viewport.rotate(rotation);
            self.viewport.draw();
            println!("{}", self.viewport);
            sleep(self.frequency);
        }
    }
}
