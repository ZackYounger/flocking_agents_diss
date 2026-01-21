use agent_lib::simulation::Simulation;
use macroquad::prelude::*;

#[macroquad::main("Splingo")]
async fn main() {
    let width = 300.0;
    let height = 300.0;
    let depth = 300.0;
    let num_agents = 100;
    
    let mut sim = Simulation::new(num_agents, width, height, depth);
    
    // Cap framerate at 60 FPS for consistent simulation speed
    let target_fps = 60;
    let frame_time = 1.0 / target_fps as f64;
    
    loop {
        let frame_start = get_time();
        
        sim.step();
        sim.draw();
        
        // Wait to maintain target framerate
        let elapsed = get_time() - frame_start;
        if elapsed < frame_time {
            let sleep_time = frame_time - elapsed;
            std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
        }
        
        next_frame().await;
    }
}
