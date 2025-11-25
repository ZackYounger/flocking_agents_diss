use crate::agent::Agent;
use macroquad::prelude::*;

pub struct Screen {
    width: f32,
    height: f32,
}

impl Screen {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn draw_agents(&self, agents: &[Box<dyn Agent>]) {
        clear_background(BLACK);
        
        // Draw agents as circles
        for agent in agents {
            let state = agent.state();
            draw_circle(state.pos.x, state.pos.y, 3.0, BLUE);
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}

