use crate::boid::Boid;
use crate::vec::Vec2;
use rand::Rng;

pub struct World {
    agents: Vec<Boid>,
    width: f32,
    height: f32,
    depth: f32,
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
        
        Self { agents, width, height, depth }
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
        // We need to collect boid data to avoid borrow checker issues
        let agent_data: Vec<Boid> = self.agents.iter().copied().collect();
        let width = self.width;
        let height = self.height;
        let depth = self.depth;
        
        for agent in &mut self.agents {
            agent.update(&agent_data, width, height, depth);
        }
        
        // Second pass: apply the updates
        for agent in &mut self.agents {
            agent.apply_update();
        }
    }
}
