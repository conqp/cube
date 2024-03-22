use crate::{Cube, Vec3d, Viewport};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Animation<'a, T, const SCALING: u8>
where
    T: Iterator<Item = Vec3d>,
{
    cube: &'a Cube<'a>,
    animator: T,
    frequency: Duration,
    viewport: Viewport<'a, SCALING>,
}

impl<'a, T, const SCALING: u8> Animation<'a, T, SCALING>
where
    T: Iterator<Item = Vec3d>,
{
    #[must_use]
    pub const fn new(
        cube: &'a Cube,
        animator: T,
        frequency: Duration,
        viewport: Viewport<'a, SCALING>,
    ) -> Self {
        Self {
            cube,
            animator,
            frequency,
            viewport,
        }
    }

    pub fn run(&mut self) {
        let mut writer = stdout().lock();

        for rotation in &mut self.animator {
            self.viewport.rotate(rotation);
            self.viewport.draw(self.cube);
            writer
                .write_all(self.viewport.to_string().as_bytes())
                .expect("could not write bytes to STDOUT");
            sleep(self.frequency);
        }
    }
}
