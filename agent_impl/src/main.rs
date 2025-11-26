use agent_lib::simulation::Simulation;
use macroquad::prelude::*;

#[macroquad::main("Splingo")]
async fn main() {
    let width = 600.0;
    let height = 600.0;
    let depth = 600.0;
    let num_agents = 50;
    
    let mut sim = Simulation::new(num_agents, width, height, depth);
    
    loop {
        sim.step();
        sim.draw();
        
        next_frame().await;
    }
}
