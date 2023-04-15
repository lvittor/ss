use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{stdout, Write},
};
use chumsky::Parser;
use cim::particles::ID;

use tp3::{
    parser::input_parser,
    particle::{Ball, InputData},
};

use clap::Parser as _parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    max_duration: Option<f64>,
}

fn run<W: Write, F: FnMut(&BTreeMap<ID, Ball>, f64) -> bool>(
    config: InputData,
    mut output_writer: W,
    mut stop_condition: F,
) {
    todo!();
    let mut time = 0.0;
    let mut state: BTreeMap<_, _> = config.balls.into_iter().map(|p| (p.id, p)).collect();

    while !stop_condition(&state, time) {}
}

fn main() {
    let args = Args::parse();

    let input = fs::read_to_string(args.input).unwrap();
    let input = input_parser()
        .parse(&input)
        .into_result()
        .expect("Error parsing input data.");

    let writer = if let Some(output) = args.output {
        Box::new(File::create(output).unwrap()) as Box<dyn Write>
    } else {
        Box::new(stdout())
    };

    run(input, writer, |_state, t| {
        args.max_duration
            .is_some_and(|max_duration| t > max_duration)
    });
}
