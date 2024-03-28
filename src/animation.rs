use crate::{Cube, Vec3d, Viewport};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Animation<'a, T>
where
    T: Iterator<Item = Vec3d>,
{
    cube: &'a Cube<'a>,
    animator: T,
    frequency: Duration,
    viewport: Viewport<'a>,
}

impl<'a, T> Animation<'a, T>
where
    T: Iterator<Item = Vec3d>,
{
    #[must_use]
    pub const fn new(
        cube: &'a Cube,
        animator: T,
        frequency: Duration,
        viewport: Viewport<'a>,
    ) -> Self {
        Self {
            cube,
            animator,
            frequency,
            viewport,
        }
    }

    /// Run the animation.
    ///
    /// # Errors
    /// Return an [`std::io::Error`] if the output cannot be written.
    pub fn run(&mut self) -> std::io::Result<()> {
        let mut writer = stdout().lock();

        for rotation in &mut self.animator {
            self.viewport.rotate(rotation);
            self.viewport.draw(self.cube);
            writer.write_all(self.viewport.to_string().as_bytes())?;
            sleep(self.frequency);
        }

        Ok(())
    }
}
