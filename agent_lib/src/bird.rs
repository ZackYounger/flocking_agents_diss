use std::fmt::Debug;
use crate::agent::{Agent, FlockAgent};
use crate::vec::Vec2;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use macroquad::prelude::Color;

//pub const LEADER_CHANCE: f64 = 0.0;
pub const LEADER_CHANCE: f64 = 0.0001;
pub const LEADER_DURATION: f32 = 3.0;
pub const LEADER_BOOST_STRENGTH: f32 = 0.025;
pub const LEADER_MAX_SPEED_MULTIPLIER: f32 = 1.1;

#[derive(Debug, Copy, Clone)]
pub struct Bird {
    pub agent: Agent,

    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub separation_weight: f32,

    pub perception_radius: f32,
    pub fov_angle: f32,

    pub min_speed: f32,
    pub max_speed: f32,
    pub speed_limit_softness: f32,
    
    pub max_neighbors: usize,

    pub eccentricity_weight: f32,
    pub is_leader: bool,
    pub leader_timer: f32,
}

impl FlockAgent for Bird {
    fn get_agent_mut(&mut self) -> &mut Agent {
        &mut self.agent
    }

    fn get_agent(&self) -> &Agent {
        &self.agent
    }

    fn update(&mut self, _all_agents: &[Agent], neighbors: &[&Agent], width: f32, height: f32, depth: f32) {

        // Limit max neihgbours by closest distance
        let nearby_agents = self.agent.get_topological_neighbors(neighbors, self.max_neighbors);
            
        let neighbors = nearby_agents.as_slice();

        let dt = 1.0 / 60.0; // Assuming 60fps

        if self.is_leader {
            self.leader_timer -= dt;
            if self.leader_timer <= 0.0 {
                self.is_leader = false;
            }
        }
        
        // Sync leadership status
        self.agent.is_leader = self.is_leader;

        // Calculate dynamic alignment and cohesion weights: inversely proportional to density
        let crowding_factor = 1.0 - (neighbors.len() as f32 / self.max_neighbors as f32).min(1.0) * 0.8;
        let effective_alignment_weight = self.alignment_weight * crowding_factor;
        let effective_cohesion_weight = self.cohesion_weight * crowding_factor;


        // Standard steering calculations
        let alignment = self.agent.alignment(neighbors) * effective_alignment_weight;
        let cohesion = self.agent.cohesion(neighbors) * effective_cohesion_weight;
        let separation = self.agent.separation(neighbors) * self.separation_weight;

        // Total steering force
        let mut steering = alignment + cohesion + separation;

        // Add speed fluctuation (Bird "flutter")
        steering = steering + self.agent.calculate_speed_flutter();
        
        // For making new leaders
        self.evaluate_leadership(neighbors);

        // Apply leadership speed boost
        if self.is_leader {

            let progress = (LEADER_DURATION - self.leader_timer) / LEADER_DURATION;
            let boost_factor = (progress * std::f32::consts::PI).sin(); // Sin curve for smooth ramp up/down
            let boost = self.agent.vel * (boost_factor * LEADER_BOOST_STRENGTH); 
            steering = steering + boost;
        }
        
        // Update velocity
        self.agent.new_vel = self.agent.vel + steering;
        
        // Speed management (leaders can exceed max_speed slightly)
        let effective_max_speed = if self.is_leader { self.max_speed * LEADER_MAX_SPEED_MULTIPLIER } else { self.max_speed };
        self.agent.limit_speed(self.min_speed, effective_max_speed, self.speed_limit_softness);
        
        self.agent.new_pos = self.agent.pos + self.agent.new_vel;
        
        // Wrap around edges
        self.agent.wrap_position(width, height, depth);
    }

    fn perception_radius(&self) -> f32 {
        self.perception_radius
    }

    fn fov_angle(&self) -> f32 {
        self.fov_angle
    }

    fn display_color(&self) -> Color {
        if self.is_leader {
            Color::from_rgba(255, 69, 0, 255)
        } else {
            Color::from_rgba(100, 149, 237, 255)
        }
    }
}

impl Bird {
    pub fn new(pos: Vec2) -> Self {
        Self {
            agent: Agent::new(pos),

            alignment_weight: 1.0,
            cohesion_weight: 0.5,
            separation_weight: 2.5,

            perception_radius: 35.0,
            fov_angle: 4.41,

            min_speed: 0.5,
            max_speed: 2.0,
            speed_limit_softness: 0.1,
            
            max_neighbors: 7, // StarFlag Project finding (6-7 neighbors)
            
            eccentricity_weight: 0.5,
            is_leader: false,
            leader_timer: 0.0,
        }
    }

    fn evaluate_leadership(&mut self, neighbors: &[&Agent]) {
        // Calculate cohesion vector (ki) and center of mass
        let mut center_of_mass = Vec2::zero();
        let mut k_i = Vec2::zero();
        
        if !neighbors.is_empty() {
            for n in neighbors {
                center_of_mass = center_of_mass + n.pos;
            }
            center_of_mass = center_of_mass / neighbors.len() as f32;
            k_i = center_of_mass - self.agent.pos;
        }

        // Leadership evaluation
        if !self.is_leader && !neighbors.is_empty() {
            
            // Calculate eccentricity (xi)
            let dist_to_center = (self.agent.pos - center_of_mass).length();
            let xi = (dist_to_center / self.perception_radius).min(1.0);

            // Evaluate if in front of flock
            let vel_norm = self.agent.vel / (self.agent.vel.length() + 0.0001);
            let k_norm = k_i / (k_i.length() + 0.0001);
            
            // Dot product determines if boid is in front of the flock
            let in_front = (-k_norm).dot(vel_norm);

            // Trigger leadership
            let mut rng = rand::thread_rng();
            let normal = Normal::new(0.5f32, 0.15f32).unwrap();
            let gauss_rand: f32 = normal.sample(&mut rng).max(0.0).min(1.0);
            if xi > self.eccentricity_weight * gauss_rand && in_front > 0.0 && rng.gen_bool(LEADER_CHANCE) {
                self.is_leader = true;
                self.leader_timer = LEADER_DURATION;
            }
        }
    }

    pub fn apply_update(&mut self) {
        self.agent.apply_update();
    }
}

