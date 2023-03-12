use std::io::{stdin, Read};

use chumsky::Parser;
use cim_implementation::{
    cim_finder::CimNeighborFinder, neighbor_finder::NeighborFinder, parser::input_parser,
    particles::ParticlesData, simple_finder::SimpleNeighborFinder,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input: ParticlesData = input_parser()
        .parse(&input)
        .into_result()
        .expect("Error parsing input data.");

    //dbg!(&input);

    //let output = SimpleNeighborFinder::find_neighbors(&input, false);
    let output = CimNeighborFinder::find_neighbors(&input, true);

    print!("{output}");
}
