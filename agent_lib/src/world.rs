use crate::agent::Agent;
use crate::fish::Fish;
use crate::vec::Vec2;
use rand::Rng;

pub struct World {
    agents: Vec<Box<dyn Agent>>,
    width: f32,
    height: f32,
}

impl World {
    pub fn new(num_agents: usize, width: f32, height: f32) -> Self {
        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();
        
        for _ in 0..num_agents {
            let x = rng.gen_range(0.0..width);
            let y = rng.gen_range(0.0..height);
            let pos = Vec2::new(x, y);
            let fish = Fish::new(pos);
            agents.push(Box::new(fish) as Box<dyn Agent>);
        }
        
        Self { agents, width, height }
    }

    pub fn add_agent(&mut self, agent: Box<dyn Agent>) {
        self.agents.push(agent);
    }

    pub fn agents(&self) -> &[Box<dyn Agent>] {
        &self.agents
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn speak_all(&self) {
        for agent in &self.agents {
            agent.speak();
        }
    }

    pub fn update(&mut self) {
        // Update world state (e.g., move agents based on velocity)
        for _agent in &mut self.agents {
            // Agents will be updated here
        }
    }
}
