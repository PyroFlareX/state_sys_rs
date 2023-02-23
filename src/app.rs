use std::boxed::Box;
use std::vec::Vec;

use crate::state;
use state::*;

pub struct App {
    pub states: Vec<Box<dyn State>>,
}

impl App {
    pub fn new() -> Self {
        Self { states: vec![] }
    }

    pub fn run_loop(&mut self) {
        let mut num_states = self.states.len();

        let mut dt = 0.0f32;

        while num_states > 0 {
            let cur_state = self.states.last_mut().unwrap();

            cur_state.input(&dt);
            cur_state.update(&dt);

            //Update loop states
            dt = 0.0f32;
            num_states = self.states.len();
        }
    }

    pub fn push_state(&mut self, state: Box<dyn State>) {
        self.states.push(state)
    }

	pub fn pop_state(&mut self) {
		let s = self.states.pop();
		// s.drop
	}
}
