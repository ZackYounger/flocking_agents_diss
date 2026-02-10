use std::fmt::Debug;


pub use crate::vec::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Agent {
    pub pos: Vec2,
    pub new_pos: Vec2,
    pub vel: Vec2,
    pub new_vel: Vec2,
    pub is_leader: bool,
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
            is_leader: false,
        }
    }

    pub fn limit_speed(&mut self, min_speed: f32, max_speed: f32, speed_limit_softness: f32) {
        let speed_sq = self.new_vel.length_sq();
        
        if speed_sq > 0.0001 {
            let speed = speed_sq.sqrt();
            if speed > max_speed {
                self.new_vel = self.new_vel / speed * max_speed;
            } else if speed < min_speed {
                // Apply a gentle force to speed up towards min_speed
                let desired_vel = self.new_vel.normalize() * min_speed;
                let steer_force = (desired_vel - self.new_vel) * speed_limit_softness; // Adjusted multiplier for stronger softness
                self.new_vel = self.new_vel + steer_force;
            }
        } else {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let angle_xy = rng.gen_range(0.0..std::f32::consts::TAU);
            let angle_z = rng.gen_range(-std::f32::consts::PI / 4.0..std::f32::consts::PI / 4.0);
            
            self.new_vel = Vec2::new_3d(
                angle_xy.cos() * angle_z.cos() * min_speed,
                angle_xy.sin() * angle_z.cos() * min_speed,
                angle_z.sin() * min_speed,
            );
        }
    }

    pub fn calculate_speed_flutter(&self) -> Vec2 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let speed_variation = rng.gen_range(-0.05..0.05);
        if self.vel.length_sq() > 0.001 {
            (self.vel / self.vel.length()) * speed_variation
        } else {
            Vec2::new(1.0, 0.0) * speed_variation
        }
    }

    pub fn get_current_speed(&self) -> f32 {
        self.vel.length()
    }



    /// Common neighbor filtering logic - applies distance and FOV checks
    pub fn filter_neighbors<'a>(
        &self,
        all_agents: &'a [Agent],
        candidate_indices: &[usize],
        perception_radius: f32,
        fov_angle: f32,
    ) -> Vec<&'a Agent> {
        // Pre-calculate values for the filter to avoid repeated work
        let vel_len = self.vel.length();
        let fov_cos = (fov_angle / 2.0).cos();
        let perception_radius_sq = perception_radius * perception_radius;

        candidate_indices.iter()
            .map(|&i| &all_agents[i])
            .filter(|agent| {
                let diff = agent.pos - self.pos;
                let distance_sq = diff.length_sq();
                
                if distance_sq >= perception_radius_sq || distance_sq == 0.0 {
                    return false;
                }

                // If agent is basically at rest, it can see in all directions
                if vel_len < 0.01 {
                    return true;
                }

                // FOV check using dot product
                let dot = self.vel.dot(diff);
                let cos_theta = dot / (vel_len * distance_sq.sqrt());
                
                cos_theta > fov_cos
            }).collect()
    }

    pub fn apply_update(&mut self) {
        self.vel = self.new_vel;
        self.pos = self.new_pos;
    }

    pub fn wrap_position(&mut self, width: f32, height: f32, depth: f32) {
        if self.new_pos.x < 0.0 {
            self.new_pos.x += width;
        } else if self.new_pos.x > width {
            self.new_pos.x -= width;
        }
        
        if self.new_pos.y < 0.0 {
            self.new_pos.y += height;
        } else if self.new_pos.y > height {
            self.new_pos.y -= height;
        }
        
        if self.new_pos.z < -depth / 2.0 {
            self.new_pos.z += depth;
        } else if self.new_pos.z > depth / 2.0 {
            self.new_pos.z -= depth;
        }
    }

    pub fn alignment(&self, neighbors: &[&Agent]) -> Vec2 {
        if neighbors.is_empty() {
            return Vec2::zero();
        }

        let mut avg_vel = Vec2::zero();
        let mut total_weight = 0.0;
        
        for neighbor in neighbors {
            let weight = if neighbor.is_leader { 3.0 } else { 1.0 };
            avg_vel = avg_vel + (neighbor.vel * weight);
            total_weight += weight;
        }
        
        if total_weight > 0.0 {
            avg_vel = avg_vel / total_weight;
        }

        // Return steering force towards average heading
        return (avg_vel - self.vel) * 0.05;
    }

    pub fn cohesion(&self, neighbors: &[&Agent]) -> Vec2 {
        if neighbors.is_empty() {
            return Vec2::zero();
        }

        let mut center_of_mass = Vec2::zero();
        let mut total_weight = 0.0;
        
        for neighbor in neighbors {
            let weight = if neighbor.is_leader { 3.0 } else { 1.0 };
            center_of_mass = center_of_mass + (neighbor.pos * weight);
            total_weight += weight;
        }
        
        if total_weight > 0.0 {
            center_of_mass = center_of_mass / total_weight;
        }

        // Steer towards center of mass with proper scaling
        return (center_of_mass - self.pos) * 0.003;
    }

    pub fn separation(&self, neighbors: &[&Agent]) -> Vec2 {
        let mut repulsion = Vec2::zero();
        for neighbor in neighbors {
            let diff = self.pos - neighbor.pos;
            let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
            
            if distance_sq > 0.0 {
                let weight = if neighbor.is_leader { 3.0 } else { 1.0 };
                repulsion = repulsion + ((diff / distance_sq) * weight);
            }
        }

        return repulsion * 0.1;
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

    /// Selects the closest 'max_neighbors' from the provided list, prioritising leaders
    pub fn get_topological_neighbors<'a>(&self, neighbors: &[&'a Agent], max_neighbors: usize) -> Vec<&'a Agent> {
        if neighbors.len() <= max_neighbors {
            return neighbors.to_vec();
        }

        let mut neighbor_pairs: Vec<(&Agent, f32)> = neighbors.iter()
            .map(|&n| (n, (self.pos - n.pos).length_sq()))
            .filter(|(_, d)| *d > 0.00001) // Exclude self
            .collect();
        
        // Leaders first
        // then closest
        if neighbor_pairs.len() > max_neighbors {
            neighbor_pairs.select_nth_unstable_by(max_neighbors, |(a1, d1), (a2, d2)| {
                match (a1.is_leader, a2.is_leader) {
                    (true, false) => std::cmp::Ordering::Less, // Leader better
                    (false, true) => std::cmp::Ordering::Greater, // Not leader worse 
                    _ => d1.partial_cmp(d2).unwrap_or(std::cmp::Ordering::Equal), // Otherwise sort by distance
                }
            });
        }
        
        neighbor_pairs.into_iter()
            .take(max_neighbors)
            .map(|(a, _)| a)
            .collect()
    }
}

pub trait FlockAgent: Send + Sync + Sized + Debug {
    fn get_agent_mut(&mut self) -> &mut Agent;
    fn get_agent(&self) -> &Agent;
    fn update(&mut self, all_agents: &[Agent], neighbors: &[&Agent], width: f32, height: f32, depth: f32);
    fn perception_radius(&self) -> f32;
    fn fov_angle(&self) -> f32;
    fn display_color(&self) -> macroquad::prelude::Color;
}
