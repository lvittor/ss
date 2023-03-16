use cgmath::vec2;
use cim::{
    cim_finder::CimNeighborFinder,
    neighbor_finder::NeighborFinder,
    particles::{Particle, ParticlesData},
};

fn main() {
    let map = CimNeighborFinder::find_neighbors(
        &ParticlesData {
            space_length: 100.0,
            grid_size: 10,
            interaction_radius: 10.0,
            particles: vec![
                Particle {
                    id: 0,
                    position: vec2(10.0, 10.0),
                    radius: 5.0,
                },
                Particle {
                    id: 1,
                    position: vec2(20.0, 10.0),
                    radius: 5.0,
                },
                Particle {
                    id: 2,
                    position: vec2(30.0, 20.0),
                    radius: 5.0,
                },
                Particle {
                    id: 3,
                    position: vec2(10.0, 40.0),
                    radius: 5.0,
                },
            ],
        },
        false,
    );

    println!("{map}");
}
