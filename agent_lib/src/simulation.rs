use crate::world::World;
use crate::screen::Screen;

pub struct Simulation {
    world: World,
    screen: Screen,
    width: f32,
    height: f32,
    depth: f32,
}

impl Simulation {
    pub fn new(num_agents: usize, width: f32, height: f32, depth: f32) -> Self {
        Self {
            world: World::new(num_agents, width, height, depth),
            screen: Screen::new(width, height, depth),
            width,
            height,
            depth,
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
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

    pub fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
            self.world.speak_all();
            self.screen.draw_agents(self.world.agents());
            println!("---");
        }
    }
}
