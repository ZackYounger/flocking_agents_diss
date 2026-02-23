use agent_lib::simulation::Simulation;
use agent_lib::world::AccelerationStructure;
use agent_lib::bird::Bird;
use agent_lib::fish::Fish;

use std::fs::File;
use std::io::Write;

fn main() {
    let width = 450.0;
    let height = 450.0;
    let depth = 450.0;
    let num_agents = 300;
    
    // Choose Params
    type CurrentAgent = Fish; // Or Fish
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
    let analysis_steps = 1000;

    // Let sim stabilise
    println!("Stabilising simulation for {} seconds", simulation_warmup_time);
    for _ in 0..(target_fps * simulation_warmup_time as usize) {
        sim.step();
    }
    println!("Simulation stabilised");
    
    println!("Collecting agent speeds for {} steps", analysis_steps);
    let mut file = File::create("agent_speeds.csv").expect("Could not create CSV file");
    for _ in 0..analysis_steps {
        sim.step();
        let speeds = sim.get_agent_speeds();
        for speed in speeds {
            writeln!(file, "{}", speed).expect("Could not write to CSV file");
        }
    }
    println!("Agent speeds saved to agent_speeds.csv");
}
