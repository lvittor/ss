use std::{fs, time::Instant};

use chumsky::Parser;
use cim::{
    cim_finder::{self, CimNeighborFinder},
    neighbor_finder::NeighborFinder,
    simple_finder::{self, SimpleNeighborFinder},
};
use clap::Parser as _parser;
use tp1::{parser::input_parser, particle::ParticlesData};

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
        SimpleNeighborFinder::find_neighbors(
            &input.particles,
            simple_finder::SystemInfo {
                cyclic: args.cyclic,
                interaction_radius: input.interaction_radius,
                space_width: input.space_length,
                space_height: input.space_length,
            },
        )
    } else {
        CimNeighborFinder::find_neighbors(
            &input.particles,
            cim_finder::SystemInfo {
                cyclic: args.cyclic,
                interaction_radius: input.interaction_radius,
                space_width: input.space_length,
                space_height: input.space_length,
                columns: input.grid_size,
                rows: input.grid_size,
            },
        )
    };
    let end = Instant::now();

    if let Some(output_file) = args.output {
        fs::write(output_file, format!("{output}")).unwrap();
    }
    let delta = (end - start).as_secs_f64();
    eprintln!("{delta}");
}
