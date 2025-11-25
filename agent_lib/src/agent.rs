use std::fmt::Debug;

pub use crate::vec::Vec2;

#[derive(Debug)]
pub struct AgentState {
    pub pos: Vec2,
    pub vel: Vec2,
    pub name: String,
}

impl AgentState {
    pub fn new(pos: Vec2, name: String) -> Self {
        Self {
            pos,
            vel: Vec2::zero(),
            name,
        }
    }
}

pub trait Agent: Debug {
    fn speak(&self);
    fn state(&self) -> &AgentState;
}