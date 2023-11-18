use crate::{Cube, Vec3d};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Viewport<'a, const K1: u8> {
    cube: &'a Cube,
    width: u32,
    height: u32,
    orientation: Vec3d,
    background: char,
    distance: u8,
    z: Vec<f64>,
    buffer: Vec<char>,
}

impl<'a, const K1: u8> Viewport<'a, K1> {
    #[must_use]
    pub fn new(
        cube: &'a Cube,
        width: u32,
        height: u32,
        orientation: Vec3d,
        background: char,
        distance: u8,
    ) -> Self {
        Self {
            cube,
            width,
            height,
            orientation,
            background,
            distance,
            z: vec![0.0; width as usize * height as usize],
            buffer: vec![background; width as usize * height as usize],
        }
    }

    pub fn rotate(&mut self, orientation: Vec3d) {
        self.orientation = orientation;
        self.draw();
    }

    fn draw(&mut self) {
        self.reset_buffers();

        for x in (-i32::from(self.cube.size()))..(self.cube.size().into()) {
            for y in (-i32::from(self.cube.size()))..(self.cube.size().into()) {
                self.draw_surface(
                    x.into(),
                    y.into(),
                    -f64::from(self.cube.size()),
                    self.cube.side(0),
                );
                self.draw_surface(
                    f64::from(self.cube.size()),
                    y.into(),
                    x.into(),
                    self.cube.side(1),
                );
                self.draw_surface(
                    -f64::from(self.cube.size()),
                    y.into(),
                    -f64::from(x),
                    self.cube.side(2),
                );
                self.draw_surface(
                    -f64::from(x),
                    y.into(),
                    self.cube.size().into(),
                    self.cube.side(3),
                );
                self.draw_surface(
                    x.into(),
                    -f64::from(self.cube.size()),
                    -f64::from(y),
                    self.cube.side(4),
                );
                self.draw_surface(
                    x.into(),
                    self.cube.size().into(),
                    y.into(),
                    self.cube.side(5),
                );
            }
        }
    }

    fn reset_buffers(&mut self) {
        for item in &mut self.z {
            *item = 0.0;
        }

        for item in &mut self.buffer {
            *item = self.background;
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn draw_surface(&mut self, x: f64, y: f64, z: f64, chr: char) {
        let ooz = 1.0 / self.gamma(x, y, z) + f64::from(self.distance);
        let xp = (f64::from(K1) * ooz * self.alpha(x, y, z))
            .mul_add(2.0, f64::from(self.width) / 2.0)
            .round();
        let yp = (f64::from(K1) * ooz)
            .mul_add(self.beta(x, y, z), f64::from(self.height) / 2.0)
            .round();
        let idx = yp.mul_add(f64::from(self.width), xp).round() as usize;

        if (idx < self.width as usize * self.height as usize) && (ooz > self.z[idx]) {
            self.z[idx] = ooz;
            self.buffer[idx] = chr;
        }
    }

    fn alpha(&self, i: f64, j: f64, k: f64) -> f64 {
        (i * self.orientation.1.cos()).mul_add(
            self.orientation.2.cos(),
            (k * self.orientation.0.sin()).mul_add(
                self.orientation.2.sin(),
                (j * self.orientation.0.cos()).mul_add(
                    self.orientation.2.sin(),
                    (j * self.orientation.0.sin() * self.orientation.1.sin()).mul_add(
                        self.orientation.2.cos(),
                        -k * self.orientation.0.cos()
                            * self.orientation.1.sin()
                            * self.orientation.2.cos(),
                    ),
                ),
            ),
        )
    }

    fn beta(&self, i: f64, j: f64, k: f64) -> f64 {
        (i * self.orientation.1.cos()).mul_add(
            -self.orientation.2.sin(),
            (k * self.orientation.0.cos() * self.orientation.1.sin()).mul_add(
                self.orientation.2.sin(),
                (j * self.orientation.0.sin() * self.orientation.1.sin()).mul_add(
                    -self.orientation.2.sin(),
                    (j * self.orientation.0.cos()).mul_add(
                        self.orientation.2.cos(),
                        k * self.orientation.0.sin() * self.orientation.2.cos(),
                    ),
                ),
            ),
        )
    }

    fn gamma(&self, i: f64, j: f64, k: f64) -> f64 {
        i.mul_add(
            self.orientation.1.sin(),
            (k * self.orientation.0.cos()).mul_add(
                self.orientation.1.cos(),
                -j * self.orientation.0.sin() * self.orientation.1.cos(),
            ),
        )
    }
}

impl<'a, const K1: u8> Display for Viewport<'a, K1> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, chr) in self.buffer.iter().enumerate() {
            if (index + 1) % self.width as usize == 0 {
                writeln!(f, "{chr}")?;
            } else {
                write!(f, "{chr}")?;
            }
        }

        Ok(())
    }
}
