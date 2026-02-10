use std::fmt::Debug;
use crate::agent::{Agent, FlockAgent};
use crate::vec::Vec2;
use macroquad::prelude::Color;

pub const LEVELING_STRENGTH : f32 = 0.5;

#[derive(Debug, Copy, Clone)]
pub struct Fish {
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
}

impl FlockAgent for Fish {
    fn get_agent_mut(&mut self) -> &mut Agent {
        &mut self.agent
    }

    fn get_agent(&self) -> &Agent {
        &self.agent
    }

    fn update(&mut self, _all_agents: &[Agent], neighbors: &[&Agent], width: f32, height: f32, depth: f32) {
        // Limit max neihgbours by closest distance
        //let nearby_agents = self.agent.get_topological_neighbors(neighbors, self.max_neighbors);

        // Standard steering calculations
        let alignment = self.agent.alignment(neighbors) * self.alignment_weight;
        let cohesion = self.agent.cohesion(neighbors) * self.cohesion_weight;
        let separation = self.agent.separation(neighbors) * self.separation_weight;

        // Total steering force
        let mut steering = alignment + cohesion + separation;
        
        // Bias to swim flat
        steering.y -= self.agent.vel.y * LEVELING_STRENGTH;

        // Update velocity
        self.agent.new_vel = self.agent.vel + steering;
        
        self.agent.limit_speed(self.min_speed, self.max_speed, self.speed_limit_softness);
        
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
        Color::from_rgba(0, 128, 128, 255)
    }
}

impl Fish {
    pub fn new(pos: Vec2) -> Self {
        Self {
            agent: Agent::new(pos),

            alignment_weight: 1.8,
            cohesion_weight: 0.8,
            separation_weight: 1.5,

            perception_radius: 40.0,
            fov_angle: 5.0, 

            min_speed: 0.5,
            max_speed: 1.8,
            speed_limit_softness: 0.5,

            max_neighbors: 4, // atm not used
        }
    }

    pub fn apply_update(&mut self) {
        self.agent.apply_update();
    }
}
