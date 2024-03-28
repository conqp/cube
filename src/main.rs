use clap::Parser;
use cube::{Animation, Cube, RandomRotation, Viewport, DEFAULT_SIZE};
use std::time::Duration;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, short, help = "cube edge size", default_value_t = DEFAULT_SIZE)]
    size: u8,
    #[arg(long, short, help = "viewport width", default_value_t = 160)]
    width: u8,
    #[arg(long, short = 'H', help = "viewport height", default_value_t = 44)]
    height: u8,
    #[arg(long, short, help = "distance from camera", default_value_t = 100)]
    distance: u8,
    #[arg(
        long,
        short = 'a',
        help = "surface sampling rate",
        default_value_t = 0.6
    )]
    sample_rate: f64,
    #[arg(
        long,
        short,
        help = "rotation frequency in milliseconds",
        default_value_t = 8
    )]
    frequency: u64,
    #[arg(long, short, help = "background character", default_value = " ")]
    background: char,
    #[arg(
        long,
        short = 'c',
        help = "scaling of the viewport",
        default_value_t = 40.0
    )]
    scaling: f64,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    Animation::new(
        &Cube::with_size(args.size),
        RandomRotation::default(),
        Duration::from_millis(args.frequency),
        Viewport::new(
            args.width,
            args.height,
            args.background.to_string().as_str(),
            args.distance,
            args.sample_rate,
            args.scaling,
        ),
    )
    .run()
}
