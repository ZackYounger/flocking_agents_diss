use std::fmt::Debug;

pub use crate::vec::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Agent {
    pub pos: Vec2,
    pub new_pos: Vec2,
    pub vel: Vec2,
    pub new_vel: Vec2,
}

impl Agent {
    pub fn new(pos: Vec2) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Start with velocity at preferred cruising speed
        let initial_speed = 2.0;  // Match the preferred speed
        let angle_xy = rng.gen_range(0.0..std::f32::consts::TAU);
        let angle_z = rng.gen_range(-std::f32::consts::PI / 4.0..std::f32::consts::PI / 4.0);
        
        let vel = Vec2::new_3d(
            angle_xy.cos() * angle_z.cos() * initial_speed,
            angle_xy.sin() * angle_z.cos() * initial_speed,
            angle_z.sin() * initial_speed,
        );
        
        Self {
            pos,
            new_pos: pos,
            vel,
            new_vel: vel,
        }
    }

    pub fn apply_update(&mut self) {
        self.vel = self.new_vel;
        self.pos = self.new_pos;
    }

    // Boid behavior functions - can be reused by different agent types
    pub fn alignment(&self, neighbors: &[&Agent]) -> Vec2 {
        if neighbors.is_empty() {
            return Vec2::zero();
        }

        let mut avg_vel = Vec2::zero();
        for neighbor in neighbors {
            avg_vel = avg_vel + neighbor.vel;
        }
        avg_vel.x /= neighbors.len() as f32;
        avg_vel.y /= neighbors.len() as f32;
        avg_vel.z /= neighbors.len() as f32;

        // Return steering force towards average heading
        return (avg_vel - self.vel) * 0.05;
    }

    pub fn cohesion(&self, neighbors: &[&Agent]) -> Vec2 {
        if neighbors.is_empty() {
            return Vec2::zero();
        }

        let mut center_of_mass = Vec2::zero();
        for neighbor in neighbors {
            center_of_mass = center_of_mass + neighbor.pos;
        }
        center_of_mass.x /= neighbors.len() as f32;
        center_of_mass.y /= neighbors.len() as f32;
        center_of_mass.z /= neighbors.len() as f32;

        // Steer towards center of mass with proper scaling
        return (center_of_mass - self.pos) * 0.003;  // Reduced to make cohesion gentler
    }

    pub fn separation(&self, neighbors: &[&Agent]) -> Vec2 {
        let mut repulsion = Vec2::zero();
        for neighbor in neighbors {
            let diff = self.pos - neighbor.pos;
            let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
            if distance_sq > 0.0 {
                repulsion = repulsion + (diff / distance_sq);
            }
        }

        return repulsion * 0.1;  // Stronger separation force
    }

    pub fn edge_avoidance(&self, width: f32, height: f32, depth: f32) -> Vec2 {
        let margin = 50.0; // Distance from edge to start applying force
        let turn_factor = 0.5; // Strength of the turning force
        let mut avoidance = Vec2::zero();

        // Check X bounds
        if self.pos.x < margin {
            avoidance.x += turn_factor;
        } else if self.pos.x > width - margin {
            avoidance.x -= turn_factor;
        }

        // Check Y bounds
        if self.pos.y < margin {
            avoidance.y += turn_factor;
        } else if self.pos.y > height - margin {
            avoidance.y -= turn_factor;
        }

        // Check Z bounds
        if self.pos.z < -depth / 2.0 + margin {
            avoidance.z += turn_factor;
        } else if self.pos.z > depth / 2.0 - margin {
            avoidance.z -= turn_factor;
        }

        avoidance
    }

    pub fn random_jitter(&self, jitter_strength: f32) -> Vec2 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        Vec2::new_3d(
            rng.gen_range(-jitter_strength..jitter_strength),
            rng.gen_range(-jitter_strength..jitter_strength),
            rng.gen_range(-jitter_strength..jitter_strength),
        )
    }
}
