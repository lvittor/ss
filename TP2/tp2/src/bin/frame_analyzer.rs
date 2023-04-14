use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use chumsky::Parser;
use clap::Parser as _parser;
use nalgebra::Vector2;
use tp2::{
    parser::{input_parser, output_parser},
    particle::Frame,
};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(short, long)]
    analysis: PathBuf,

    #[arg(long)]
    capture_directory: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let input = read_to_string(args.input).unwrap();
    let output_file = File::open(args.output).unwrap();
    let system_info = input_parser()
        .parse(&input)
        .into_result()
        .expect("Error parsing input data.");

    let mut analysis_file = File::create(args.analysis).unwrap();

    for frame in output_parser(
        system_info.particles.len(),
        BufReader::new(output_file).lines(),
    ) {
        let Frame { time, particles } = frame;
        let va = particles
            .iter()
            .map(|p| p.velocity_direction)
            .sum::<Vector2<f64>>()
            .magnitude()
            / particles.len() as f64;

        analysis_file
            .write_fmt(format_args!("{time},{va}\n"))
            .unwrap();
    }
}
