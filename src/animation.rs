use crate::{Cube, Vec3d, Viewport};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Animation<'a, T, const K1: u8>
where
    T: Iterator<Item = Vec3d>,
{
    cube: &'a Cube<'a>,
    animator: T,
    frequency: Duration,
    viewport: Viewport<'a, K1>,
}

impl<'a, T, const K1: u8> Animation<'a, T, K1>
where
    T: Iterator<Item = Vec3d>,
{
    #[must_use]
    pub const fn new(
        cube: &'a Cube,
        animator: T,
        frequency: Duration,
        viewport: Viewport<'a, K1>,
    ) -> Self {
        Self {
            cube,
            animator,
            frequency,
            viewport,
        }
    }

    pub fn run(&mut self) {
        for rotation in &mut self.animator {
            self.viewport.rotate(rotation);
            self.viewport.draw(self.cube);
            println!("{}", self.viewport);
            sleep(self.frequency);
        }
    }
}
