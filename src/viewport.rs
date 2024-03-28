use crate::{Cube, FloatRange, Vec3d};
use itertools::Itertools;
use std::cell::{Ref, RefCell};
use std::f64::consts::TAU;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Viewport<'a> {
    width: u8,
    height: u8,
    background: &'a str,
    distance: u8,
    sample_rate: f64,
    scaling: f64,
    orientation: Vec3d,
    buffer: Vec<(f64, &'a str)>,
    repr: RefCell<String>,
}

impl<'a> Viewport<'a> {
    #[must_use]
    pub fn new(
        width: u8,
        height: u8,
        background: &'a str,
        distance: u8,
        sample_rate: f64,
        scaling: f64,
    ) -> Self {
        Self {
            width,
            height,
            background,
            distance,
            sample_rate,
            scaling,
            orientation: Vec3d::default(),
            buffer: vec![(0.0, background); usize::from(width) * usize::from(height)],
            repr: RefCell::new(String::with_capacity(
                (usize::from(width) + 1) * usize::from(height),
            )),
        }
    }

    pub fn rotate(&mut self, rotation: Vec3d) {
        self.orientation += rotation;
        self.orientation %= TAU;
    }

    pub fn draw(&mut self, cube: &Cube<'a>) {
        self.reset_buffer();
        self.draw_surfaces(cube);
    }

    fn reset_buffer(&mut self) {
        self.buffer
            .iter_mut()
            .for_each(|item| *item = (0.0, self.background));
    }

    fn draw_surfaces(&mut self, cube: &Cube<'a>) {
        FloatRange::new(
            -f64::from(cube.size()),
            cube.size().into(),
            self.sample_rate,
        )
        .cartesian_product(FloatRange::new(
            -f64::from(cube.size()),
            cube.size().into(),
            self.sample_rate,
        ))
        .for_each(|(x, y)| {
            self.draw_surface(Vec3d::new(x, y, -f64::from(cube.size())), cube.side(0));
            self.draw_surface(Vec3d::new(f64::from(cube.size()), y, x), cube.side(1));
            self.draw_surface(Vec3d::new(-f64::from(cube.size()), y, -x), cube.side(2));
            self.draw_surface(Vec3d::new(-x, y, cube.size().into()), cube.side(3));
            self.draw_surface(Vec3d::new(x, -f64::from(cube.size()), -y), cube.side(4));
            self.draw_surface(Vec3d::new(x, cube.size().into(), y), cube.side(5));
        });
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn draw_surface(&mut self, other: Vec3d, repr: &'a str) {
        let (x, y, z) = self.orientation.angle(other).into();
        let ooz = 1.0 / (z + f64::from(self.distance));
        let xp = (self.scaling * ooz * x)
            .mul_add(2.0, f64::from(self.width) / 2.0)
            .round();
        let yp = (self.scaling * ooz)
            .mul_add(y, f64::from(self.height) / 2.0)
            .round();
        let idx = yp.mul_add(f64::from(self.width), xp).round() as usize;

        if (idx < usize::from(self.width) * usize::from(self.height)) && (ooz > self.buffer[idx].0)
        {
            self.buffer[idx] = (ooz, repr);
        }
    }

    fn buffer_str(&self) {
        let mut buffer = self.repr.borrow_mut();
        buffer.clear();
        self.buffer
            .iter()
            .map(|(_, repr)| repr)
            .enumerate()
            .for_each(|(index, repr)| {
                buffer.push_str(repr);

                if (index + 1) % self.width as usize == 0 {
                    buffer.push('\n');
                }
            });
    }

    fn as_str(&self) -> Ref<'_, String> {
        self.buffer_str();
        self.repr.borrow()
    }
}

impl Display for Viewport<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
