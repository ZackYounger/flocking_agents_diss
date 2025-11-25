use std::fmt::Debug;
use crate::agent::{Agent, AgentState};
use crate::vec::Vec2;

#[derive(Debug)]
pub struct Fish {
    pub state: AgentState,
}

impl Fish {
    pub fn new(pos: Vec2) -> Self {
        Self {
            state: AgentState::new(pos, "".to_string()),
        }
    }
}

impl Agent for Fish {
    fn speak(&self) {
        println!("glub glub");
    }

    fn state(&self) -> &AgentState {
        &self.state
    }
}
