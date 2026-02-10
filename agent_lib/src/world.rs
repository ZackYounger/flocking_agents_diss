use crate::agent::{Agent, FlockAgent};
use crate::vec::Vec2;
use crate::spatial_hash::{SpatialHash, AccelerationStructureTrait};
use crate::octree::Octree;
use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

/// Acceleration structure type for neighbour queries
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelerationStructure {
    SpatialHash,
    Octree,
}

impl Default for AccelerationStructure {
    fn default() -> Self {
        AccelerationStructure::Octree
    }
}

pub struct World<A: FlockAgent> {
    agents: Vec<A>,
    width: f32,
    height: f32,
    depth: f32,
    acceleration_structure_type: AccelerationStructure,
}

impl<A: FlockAgent> World<A> {
    pub fn new<F>(num_agents: usize, width: f32, height: f32, depth: f32, agent_constructor: F) -> Self
    where
        F: Fn(Vec2) -> A + Send + Sync,
    {
        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();
        
        // Initialise agents with random positions and velocities
        for _ in 0..num_agents {
            let x = rng.gen_range(0.0..width);
            let y = rng.gen_range(0.0..height);
            let z = rng.gen_range(-depth / 2.0..depth / 2.0);
            let pos = Vec2::new_3d(x, y, z);
            agents.push(agent_constructor(pos));
        }
        
        Self { 
            agents, 
            width, 
            height, 
            depth,
            acceleration_structure_type: AccelerationStructure::default(),
        }
    }

    // Create a world with a specific acceleration structure
    pub fn with_acceleration_structure<F>(
        num_agents: usize, 
        width: f32, 
        height: f32, 
        depth: f32,
        accel: AccelerationStructure,
        agent_constructor: F,
    ) -> Self
    where
        F: Fn(Vec2) -> A + Send + Sync,
    {
        let mut world = Self::new(num_agents, width, height, depth, agent_constructor);
        world.acceleration_structure_type = accel;
        world
    }

    pub fn set_acceleration_structure(&mut self, accel: AccelerationStructure) {
        self.acceleration_structure_type = accel;
    }

    pub fn acceleration_structure_type(&self) -> AccelerationStructure {
        self.acceleration_structure_type
    }

    pub fn add_agent(&mut self, agent: A) {
        self.agents.push(agent);
    }

    pub fn agents(&self) -> &[A] {
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
        // Collect agent data for neighbour queries
        let agent_data = self.agents.iter().map(|a| *a.get_agent()).collect::<Vec<Agent>>();
        let agent_positions: Vec<Vec2> = agent_data.iter().map(|a| a.pos).collect();

        let width = self.width;
        let height = self.height;
        let depth = self.depth;

        // Build the acceleration structure
        let mut acceleration_structure: Box<dyn AccelerationStructureTrait> = match self.acceleration_structure_type {
            AccelerationStructure::SpatialHash => Box::new(SpatialHash::new(35.0)),
            AccelerationStructure::Octree => Box::new(Octree::new(width, height, depth, 8, 10)),
        };
        acceleration_structure.build(&agent_positions);

        self.agents.par_iter_mut().for_each(|agent| {
            let perception_radius = agent.perception_radius();
            let fov_angle = agent.fov_angle();
            let agent_core = agent.get_agent();

            let neighbor_indices = acceleration_structure.query(agent_core.pos, perception_radius);
            let neighbors = agent_core.filter_neighbors(&agent_data, &neighbor_indices, perception_radius, fov_angle);
            
            agent.update(&agent_data, &neighbors, width, height, depth);
            agent.get_agent_mut().apply_update();
        });
    }

    pub fn collect_query_times_for_n_steps(&mut self, steps: usize) -> Vec<u128> {
        let mut all_query_times: Vec<u128> = Vec::new();
        let width = self.width;
        let height = self.height;
        let depth = self.depth;

        for _ in 0..steps {
            let agent_data = self.agents.iter().map(|a| *a.get_agent()).collect::<Vec<Agent>>();
            let agent_positions: Vec<Vec2> = agent_data.iter().map(|a| a.pos).collect();

            let mut acceleration_structure: Box<dyn AccelerationStructureTrait> = match self.acceleration_structure_type {
                AccelerationStructure::SpatialHash => Box::new(SpatialHash::new(35.0)),
                AccelerationStructure::Octree => Box::new(Octree::new(width, height, depth, 8, 10)),
            };
            acceleration_structure.build(&agent_positions);

            let query_times_for_step: Vec<u128> = self.agents.par_iter_mut().map(|agent| {
                let perception_radius = agent.perception_radius();
                let fov_angle = agent.fov_angle();
                let agent_core = agent.get_agent();

                let start_time = Instant::now();
                let neighbor_indices = acceleration_structure.query(agent_core.pos, perception_radius);
                let end_time = Instant::now();
                let query_time = end_time.duration_since(start_time).as_nanos();

                let neighbors = agent_core.filter_neighbors(&agent_data, &neighbor_indices, perception_radius, fov_angle);
                
                agent.update(&agent_data, &neighbors, width, height, depth);

                query_time
            }).collect();
            all_query_times.extend(query_times_for_step);
        }
        all_query_times
    }
}
