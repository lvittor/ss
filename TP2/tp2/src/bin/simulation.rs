use std::fs;

use chumsky::Parser;
use cim::{cim_finder::CimNeighborFinder, neighbor_finder::NeighborFinder};
use tp2::parser::input_parser;

use clap::Parser as _parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let input = fs::read_to_string(args.input).unwrap();
    let input = input_parser()
        .parse(&input)
        .into_result()
        .expect("Error parsing input data.");

    let m = (input.space_length / input.interaction_radius).floor() as usize;
    let neighbors = CimNeighborFinder::find_neighbors(
        &input.particles,
        cim::cim_finder::SystemInfo {
            cyclic: true,
            interaction_radius: input.interaction_radius,
            space_length: input.space_length,
            grid_size: m,
        },
    );

    dbg!(neighbors);
}
