use std::fmt::Debug;
use crate::agent::Agent;
use crate::vec::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Boid {
    pub agent: Agent,
    // Weights for the three boid rules
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub separation_weight: f32,
}

impl Boid {
    pub fn new(pos: Vec2) -> Self {
        Self {
            agent: Agent::new(pos),
            alignment_weight: 1.0,
            cohesion_weight: 0.5,
            separation_weight: 3.0,
        }
    }

    pub fn update(&mut self, all_boids: &[Boid], width: f32, height: f32, depth: f32) {
        // Filter neighbors within perception radius
        let perception_radius = 50.0;
        let neighbors: Vec<&Agent> = all_boids.iter()
            .map(|b| &b.agent)
            .filter(|agent| {
                let diff = self.agent.pos - agent.pos;
                let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
                distance_sq < perception_radius * perception_radius && distance_sq > 0.0
            }).collect();

        // Use the reusable boid behavior functions from Agent with weights
        let alignment = self.agent.alignment(&neighbors) * self.alignment_weight;
        let cohesion = self.agent.cohesion(&neighbors) * self.cohesion_weight;
        let separation = self.agent.separation(&neighbors) * self.separation_weight;
        let edge_avoidance = self.agent.edge_avoidance(width, height, depth);
        
        // Add random jitter to prevent perfect equilibrium
        let jitter = self.random_jitter();

        // Apply moderate drag to current velocity
        let drag = 0.98; 
        let damped_vel = self.agent.vel * drag;
        
        self.agent.new_vel = damped_vel + alignment + cohesion + separation + edge_avoidance + jitter;
        
        // Apply both min and max speed limits for stability
        let min_speed = 0.5;
        let max_speed = 3.0;
        let speed_sq = self.agent.new_vel.x * self.agent.new_vel.x 
                     + self.agent.new_vel.y * self.agent.new_vel.y 
                     + self.agent.new_vel.z * self.agent.new_vel.z;
        
        if speed_sq > 0.0001 {  // Avoid division by zero
            let speed = speed_sq.sqrt();
            if speed > max_speed {
                self.agent.new_vel = self.agent.new_vel / speed * max_speed;
            } else if speed < min_speed {
                self.agent.new_vel = self.agent.new_vel / speed * min_speed;
            }
        }
        
        self.agent.new_pos = self.agent.pos + self.agent.new_vel;
    }

    fn random_jitter(&self) -> Vec2 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Small random force to add variation
        let jitter_strength = 0.1;
        Vec2::new_3d(
            rng.gen_range(-jitter_strength..jitter_strength),
            rng.gen_range(-jitter_strength..jitter_strength),
            rng.gen_range(-jitter_strength..jitter_strength),
        )
    }

    pub fn apply_update(&mut self) {
        self.agent.apply_update();
    }
}
