use agent_lib::simulation::Simulation;
use agent_lib::world::AccelerationStructure;
use agent_lib::bird::Bird;

use std::fs::File;
use std::io::Write;

fn main() {
    let width = 450.0;
    let height = 450.0;
    let depth = 450.0;

    let num_agents_range = (200..=2000).step_by(200);
    let analysis_steps = 1000; 
    let simulation_warmup_time = 20.0; 
    let target_fps = 60;

    let mut octree_avg_times: Vec<(usize, f64)> = Vec::new();
    let mut spatial_hash_avg_times: Vec<(usize, f64)> = Vec::new();

    // Octree simulation
    for num_agents in num_agents_range.clone() {
        println!("Running simulation for {} agents with Octree", num_agents);
        let mut sim = Simulation::with_acceleration_structure(
            num_agents,
            width,
            height,
            depth,
            AccelerationStructure::Octree,
            Bird::new,
        );

        // Warmup simulation
        for _ in 0..(target_fps * simulation_warmup_time as usize) {
            sim.step();
        }

        let query_times = sim.collect_query_times_for_n_steps(analysis_steps);
        let total_time: u128 = query_times.iter().sum();
        let avg_time = total_time as f64 / query_times.len() as f64;
        octree_avg_times.push((num_agents, avg_time));
    }

    // SpatialHash simulation
    for num_agents in num_agents_range.clone() {
        println!("Running simulation for {} agents with SpatialHash", num_agents);
        let mut sim = Simulation::with_acceleration_structure(
            num_agents,
            width,
            height,
            depth,
            AccelerationStructure::SpatialHash,
            Bird::new,
        );

        // Warmup simulation
        for _ in 0..(target_fps * simulation_warmup_time as usize) {
            sim.step();
        }

        let query_times = sim.collect_query_times_for_n_steps(analysis_steps);
        let total_time: u128 = query_times.iter().sum();
        let avg_time = total_time as f64 / query_times.len() as f64;
        spatial_hash_avg_times.push((num_agents, avg_time));
    }

    // Save Octree average times to CSV
    let mut octree_file = File::create("octree_avg_query_times.csv").expect("Could not create octree_avg_query_times.csv");
    writeln!(octree_file, "n,time").expect("Could not write header to octree_avg_query_times.csv");
    for (n, time) in octree_avg_times {
        writeln!(octree_file, "{},{}", n, time).expect("Could not write data to octree_avg_query_times.csv");
    }
    println!("Octree average query times saved to octree_avg_query_times.csv");

    // Save SpatialHash average times to CSV
    let mut spatial_hash_file = File::create("spatial_hash_avg_query_times.csv").expect("Could not create spatial_hash_avg_query_times.csv");
    writeln!(spatial_hash_file, "n,time").expect("Could not write header to spatial_hash_avg_query_times.csv");
    for (n, time) in spatial_hash_avg_times {
        writeln!(spatial_hash_file, "{},{}", n, time).expect("Could not write data to spatial_hash_avg_query_times.csv");
    }
    println!("SpatialHash average query times saved to spatial_hash_avg_query_times.csv");
}