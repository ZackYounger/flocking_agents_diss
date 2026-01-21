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
    pub perception_radius: f32,
}

impl Boid {
    pub fn new(pos: Vec2) -> Self {
        Self {
            agent: Agent::new(pos),
            // Adjusted weights to reduce clumping
            alignment_weight: 1.0,
            cohesion_weight: 0.5,    // Reduced cohesion
            separation_weight: 2.5,  // Stronger separation to maintain spacing
            perception_radius: 35.0,
        }
    }

    pub fn update(&mut self, all_boids: &[Boid], width: f32, height: f32, depth: f32) {

        let neighbors: Vec<&Agent> = all_boids.iter()
            .map(|b| &b.agent)
            .filter(|agent| {
                let diff = self.agent.pos - agent.pos;
                let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
                distance_sq < self.perception_radius * self.perception_radius && distance_sq > 0.0
            }).collect();

        // Calculate steering forces with weights (using same neighbors for all three forces)
        let alignment = self.agent.alignment(&neighbors) * self.alignment_weight;
        let cohesion = self.agent.cohesion(&neighbors) * self.cohesion_weight;
        let separation = self.agent.separation(&neighbors) * self.separation_weight;

        // Calculate total steering force (no edge avoidance - using wrapping instead)
        let steering = alignment + cohesion + separation;
        
        // Update velocity with steering force
        self.agent.new_vel = self.agent.vel + steering;
        
        // Speed management: maintain preferred cruising speed
        let max_speed = 4.0;
        let min_speed = 1.0;
        
        let speed_sq = self.agent.new_vel.x * self.agent.new_vel.x 
                     + self.agent.new_vel.y * self.agent.new_vel.y 
                     + self.agent.new_vel.z * self.agent.new_vel.z;
        
        if speed_sq > 0.0001 {  // Avoid division by zero
            let speed = speed_sq.sqrt();
            
            // Clamp to min/max speed only
            if speed > max_speed {
                self.agent.new_vel = self.agent.new_vel / speed * max_speed;
            } else if speed < min_speed {
                self.agent.new_vel = self.agent.new_vel / speed * min_speed;
            }
        } else {
            // If somehow stopped, give a random direction at min speed
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let angle_xy = rng.gen_range(0.0..std::f32::consts::TAU);
            let angle_z = rng.gen_range(-std::f32::consts::PI / 4.0..std::f32::consts::PI / 4.0);
            
            self.agent.new_vel = Vec2::new_3d(
                angle_xy.cos() * angle_z.cos() * min_speed,
                angle_xy.sin() * angle_z.cos() * min_speed,
                angle_z.sin() * min_speed,
            );
        }
        
        self.agent.new_pos = self.agent.pos + self.agent.new_vel;
        
        // Wrap around edges (toroidal space)
        if self.agent.new_pos.x < 0.0 {
            self.agent.new_pos.x += width;
        } else if self.agent.new_pos.x > width {
            self.agent.new_pos.x -= width;
        }
        
        if self.agent.new_pos.y < 0.0 {
            self.agent.new_pos.y += height;
        } else if self.agent.new_pos.y > height {
            self.agent.new_pos.y -= height;
        }
        
        if self.agent.new_pos.z < -depth / 2.0 {
            self.agent.new_pos.z += depth;
        } else if self.agent.new_pos.z > depth / 2.0 {
            self.agent.new_pos.z -= depth;
        }
    }



    pub fn apply_update(&mut self) {
        self.agent.apply_update();
    }
}
