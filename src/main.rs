use clap::Parser;
use cube::{Animation, Cube, RandomRotation, Viewport, DEFAULT_SIZE};
use std::time::Duration;

const K1: u8 = 40;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, help = "cube edge size", default_value_t = DEFAULT_SIZE)]
    size: u8,
    #[arg(short, long, help = "viewport width", default_value_t = 160)]
    width: u8,
    #[arg(long, help = "viewport height", default_value_t = 44)]
    height: u8,
    #[arg(long, help = "distance from camera", default_value_t = 100)]
    distance: u8,
    #[arg(long, help = "surface sampling rate", default_value_t = 0.6)]
    sample_rate: f64,
    #[arg(long, help = "rotation frequency in milliseconds", default_value_t = 8)]
    frequency: u64,
}

fn main() {
    let args = Args::parse();
    let cube = Cube::with_size(args.size);
    let mut animation = Animation::new(
        RandomRotation::default(),
        Duration::from_millis(args.frequency),
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
