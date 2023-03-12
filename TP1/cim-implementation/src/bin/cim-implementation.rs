use std::{
    fs,
    io::{stdin, Read},
    time::Instant,
};

use chumsky::Parser;
use cim_implementation::{
    cim_finder::CimNeighborFinder, neighbor_finder::NeighborFinder, parser::input_parser,
    particles::ParticlesData, simple_finder::SimpleNeighborFinder,
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
    cyclic: bool,

    #[arg(short, long)]
    brute_force: bool,
}

fn main() {
    let args = Args::parse();

    let input = fs::read_to_string(args.input).unwrap();
    let input: ParticlesData = input_parser()
        .parse(&input)
        .into_result()
        .expect("Error parsing input data.");

    let start = Instant::now();
    let output = if args.brute_force {
        SimpleNeighborFinder::find_neighbors(&input, args.cyclic)
    } else {
        CimNeighborFinder::find_neighbors(&input, args.cyclic)
    };
    let end = Instant::now();

    if let Some(output_file) = args.output {
        fs::write(output_file, format!("{output}")).unwrap();
    }
    let delta = end - start;
    eprintln!("{delta:?}");
}
