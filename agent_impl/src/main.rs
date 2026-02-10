use agent_lib::simulation::Simulation;
use agent_lib::world::AccelerationStructure;
use agent_lib::bird::Bird;
use agent_lib::fish::Fish;

use macroquad::prelude::*;

#[macroquad::main("Agent Library")]
async fn main() {
    let width = 450.0;
    let height = 450.0;
    let depth = 450.0;
    let num_agents = 300;
    
    // Choose Params
    type CurrentAgent = Bird; // Or Bird
    let chosen_acceleration_structure = AccelerationStructure::Octree; // Or SpatialHash

    let mut sim = Simulation::with_acceleration_structure(
        num_agents,
        width,
        height,
        depth,
        chosen_acceleration_structure,
        CurrentAgent::new,
    );
    
    let target_fps = 60;

    let simulation_warmup_time = 20.0;

    // Let sim stabilise
    println!("Stabilising simulation for {} seconds", simulation_warmup_time);
    for _ in 0..(target_fps * simulation_warmup_time as usize) {
        sim.step();
    }
    println!("Simulation stabilised");
    
    loop {
        sim.step();
        sim.draw();
        
        next_frame().await;
    }
}
