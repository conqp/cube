use clap::Parser;
use cube::{Animation, Cube, RandomRotation, Viewport};
use std::time::Duration;

const K1: u8 = 40;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value_t = 160)]
    width: u8,
    #[arg(long, default_value_t = 44)]
    height: u8,
    #[arg(long, default_value_t = 100)]
    distance: u8,
    #[arg(long, default_value_t = 0.6)]
    sample_rate: f64,
    #[arg(long, default_value_t = 8)]
    speed: u64,
}

fn main() {
    let args = Args::parse();
    let cube = Cube::default();
    let mut animation = Animation::new(
        RandomRotation::default(),
        Duration::from_millis(args.speed),
        Viewport::<K1>::new(
            &cube,
            args.width,
            args.height,
            ".",
            args.distance,
            args.sample_rate,
        ),
    );
    animation.run();
}
