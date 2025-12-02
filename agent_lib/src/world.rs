use crate::boid::Boid;
use crate::vec::Vec2;
use rand::Rng;
use std::sync::Arc;
use std::thread;

pub struct World {
    agents: Vec<Boid>,
    width: f32,
    height: f32,
    depth: f32,
    num_threads: usize,
}

impl World {
    pub fn new(num_agents: usize, width: f32, height: f32, depth: f32) -> Self {
        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();
        
        for _ in 0..num_agents {
            let x = rng.gen_range(0.0..width);
            let y = rng.gen_range(0.0..height);
            let z = rng.gen_range(-depth / 2.0..depth / 2.0);
            let pos = Vec2::new_3d(x, y, z);
            agents.push(Boid::new(pos));
        }
        
        // Use number of logical CPU cores, or default to 4
        let num_threads = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        
        Self { agents, width, height, depth, num_threads }
    }

    pub fn add_agent(&mut self, agent: Boid) {
        self.agents.push(agent);
    }

    pub fn agents(&self) -> &[Boid] {
        &self.agents
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn depth(&self) -> f32 {
        self.depth
    }

    pub fn update(&mut self) {
        // First pass: calculate new positions/velocities for all boids
        // Share read-only agent data across all threads
        let agent_data = Arc::new(self.agents.iter().copied().collect::<Vec<Boid>>());
        let width = self.width;
        let height = self.height;
        let depth = self.depth;
        
        // Calculate chunk size for load distribution
        let total_agents = self.agents.len();
        let chunk_size = (total_agents + self.num_threads - 1) / self.num_threads;
        
        // Process chunks in parallel using scoped threads
        // This is safe because scoped threads guarantee they complete before update() returns
        thread::scope(|s| {
            for chunk in self.agents.chunks_mut(chunk_size) {
                let agent_data = Arc::clone(&agent_data);
                
                s.spawn(move || {
                    for agent in chunk {
                        agent.update(&agent_data, width, height, depth);
                    }
                });
            }
            // Scope automatically joins all threads when it ends
        });
        
        // Second pass: apply the updates (also parallelized)
        thread::scope(|s| {
            for chunk in self.agents.chunks_mut(chunk_size) {
                s.spawn(move || {
                    for agent in chunk {
                        agent.apply_update();
                    }
                });
            }
        });
    }
}
