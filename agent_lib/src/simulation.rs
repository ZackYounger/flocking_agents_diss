use crate::world::{World, AccelerationStructure};
use crate::screen::Screen;
use crate::agent::FlockAgent;
use crate::vec::Vec2;

pub struct Simulation<A: FlockAgent> {
    world: World<A>,
    screen: Screen,
    width: f32,
    height: f32,
    depth: f32,
}

impl<A: FlockAgent> Simulation<A> {
    pub fn new<F>(num_agents: usize, width: f32, height: f32, depth: f32, agent_constructor: F) -> Self
    where
        F: Fn(Vec2) -> A + Send + Sync,
    {
        Self {
            world: World::new(num_agents, width, height, depth, agent_constructor),
            screen: Screen::new(width, height, depth),
            width,
            height,
            depth,
        }
    }

    // Create a simulation with a specific acceleration structure
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
        Self {
            world: World::with_acceleration_structure(num_agents, width, height, depth, accel, agent_constructor),
            screen: Screen::new(width, height, depth),
            width,
            height,
            depth,
        }
    }

    pub fn world(&self) -> &World<A> {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World<A> {
        &mut self.world
    }

    pub fn collect_query_times_for_n_steps(&mut self, steps: usize) -> Vec<u128> {
        self.world.collect_query_times_for_n_steps(steps)
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

    pub fn step(&mut self) {
        // Update agents
        self.world.update();
    }

    pub fn draw(&self) {
        self.screen.draw_agents(self.world.agents());
    }

    pub fn get_agent_speeds(&self) -> Vec<f32> {
        self.world.agents().iter().map(|agent| agent.get_agent().get_current_speed()).collect()
    }
}
