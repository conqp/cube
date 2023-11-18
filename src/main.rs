use clap::Parser;
use cube::{Animation, Cube, RandomRotation, Vec3d, Viewport};
use std::time::Duration;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value_t = 160)]
    width: u32,
    #[arg(long, default_value_t = 44)]
    height: u32,
    #[arg(long, default_value_t = 100)]
    distance: u8,
}

fn main() {
    let args = Args::parse();
    let cube = Cube::default();
    let mut animation = Animation::new(
        RandomRotation::default(),
        Duration::from_millis(8),
        Viewport::<40>::new(
            &cube,
            args.width,
            args.height,
            Vec3d::default(),
            '.',
            args.distance,
            0.5,
        ),
    );
    animation.run();
}
