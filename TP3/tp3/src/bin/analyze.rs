use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use chumsky::Parser;
use clap::Parser as _parser;
use tp3::{
    parser::{input_parser, output_parser},
    particle::Frame,
    Float,
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

    for frame in output_parser(BufReader::new(output_file).lines()) {
        let Frame { time, balls } = frame;
        let energy: Float = balls
            .iter()
            .map(|p| 0.5 * system_info.ball_mass * p.velocity.magnitude().powi(2))
            .sum();
        let ball_count = balls.len();

        analysis_file
            .write_fmt(format_args!("{time},{ball_count},{energy}\n"))
            .unwrap();
    }
}
