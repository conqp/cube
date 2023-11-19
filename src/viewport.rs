use crate::{Cube, FloatRange, Vec3d};
use std::f64::consts::TAU;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Viewport<'a, const K1: u8> {
    cube: &'a Cube<'a>,
    width: u8,
    height: u8,
    background: &'a str,
    distance: u8,
    sample_rate: f64,
    orientation: Vec3d,
    buffer: Vec<(f64, &'a str)>,
}

impl<'a, const K1: u8> Viewport<'a, K1> {
    #[must_use]
    pub fn new(
        cube: &'a Cube<'a>,
        width: u8,
        height: u8,
        background: &'a str,
        distance: u8,
        sample_rate: f64,
    ) -> Self {
        Self {
            cube,
            width,
            height,
            background,
            distance,
            sample_rate,
            orientation: Vec3d::default(),
            buffer: vec![(0.0, background); usize::from(width) * usize::from(height)],
        }
    }

    pub fn rotate(&mut self, rotation: Vec3d) {
        self.orientation += rotation;
        self.orientation %= TAU;
    }

    pub fn draw(&mut self) {
        self.reset_buffers();

        for x in FloatRange::new(
            -f64::from(self.cube.size()),
            self.cube.size().into(),
            self.sample_rate,
        ) {
            for y in FloatRange::new(
                -f64::from(self.cube.size()),
                self.cube.size().into(),
                self.sample_rate,
            ) {
                self.draw_surface(x, y, -f64::from(self.cube.size()), self.cube.side(0));
                self.draw_surface(f64::from(self.cube.size()), y, x, self.cube.side(1));
                self.draw_surface(-f64::from(self.cube.size()), y, -x, self.cube.side(2));
                self.draw_surface(-x, y, self.cube.size().into(), self.cube.side(3));
                self.draw_surface(x, -f64::from(self.cube.size()), -y, self.cube.side(4));
                self.draw_surface(x, self.cube.size().into(), y, self.cube.side(5));
            }
        }
    }

    fn reset_buffers(&mut self) {
        for item in &mut self.buffer {
            *item = (0.0, self.background);
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn draw_surface(&mut self, x: f64, y: f64, z: f64, repr: &'a str) {
        let ooz = 1.0 / (self.gamma(x, y, z) + f64::from(self.distance));
        let xp = (f64::from(K1) * ooz * self.alpha(x, y, z))
            .mul_add(2.0, f64::from(self.width) / 2.0)
            .round();
        let yp = (f64::from(K1) * ooz)
            .mul_add(self.beta(x, y, z), f64::from(self.height) / 2.0)
            .round();
        let idx = yp.mul_add(f64::from(self.width), xp).round() as usize;

        if (idx < usize::from(self.width) * usize::from(self.height)) && (ooz > self.buffer[idx].0)
        {
            self.buffer[idx] = (ooz, repr);
        }
    }

    fn alpha(&self, i: f64, j: f64, k: f64) -> f64 {
        (i * self.orientation.y().cos()).mul_add(
            self.orientation.z().cos(),
            (k * self.orientation.x().sin()).mul_add(
                self.orientation.z().sin(),
                (j * self.orientation.x().cos()).mul_add(
                    self.orientation.z().sin(),
                    (j * self.orientation.x().sin() * self.orientation.y().sin()).mul_add(
                        self.orientation.z().cos(),
                        -k * self.orientation.x().cos()
                            * self.orientation.y().sin()
                            * self.orientation.z().cos(),
                    ),
                ),
            ),
        )
    }

    fn beta(&self, i: f64, j: f64, k: f64) -> f64 {
        (i * self.orientation.y().cos()).mul_add(
            -self.orientation.z().sin(),
            (k * self.orientation.x().cos() * self.orientation.y().sin()).mul_add(
                self.orientation.z().sin(),
                (j * self.orientation.x().sin() * self.orientation.y().sin()).mul_add(
                    -self.orientation.z().sin(),
                    (j * self.orientation.x().cos()).mul_add(
                        self.orientation.z().cos(),
                        k * self.orientation.x().sin() * self.orientation.z().cos(),
                    ),
                ),
            ),
        )
    }

    fn gamma(&self, i: f64, j: f64, k: f64) -> f64 {
        i.mul_add(
            self.orientation.y().sin(),
            (k * self.orientation.x().cos()).mul_add(
                self.orientation.y().cos(),
                -j * self.orientation.x().sin() * self.orientation.y().cos(),
            ),
        )
    }

    fn string_size(&self) -> usize {
        // Compensate for newline character at the end of each line
        (usize::from(self.width) + 1) * usize::from(self.height)
    }
}

impl<const K1: u8> Display for Viewport<'_, K1> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity(self.string_size());

        for (index, (_, repr)) in self.buffer.iter().enumerate() {
            s.push_str(repr);

            if (index + 1) % self.width as usize == 0 {
                s.push('\n');
            }
        }

        write!(f, "{s}")
    }
}
