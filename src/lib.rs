mod animation;
mod cube;
mod float_range;
mod random_rotation;
mod viewport;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3d(f64, f64, f64);

pub use animation::Animation;
pub use cube::Cube;
pub use float_range::FloatRange;
pub use random_rotation::RandomRotation;
pub use viewport::Viewport;
