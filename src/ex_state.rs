#![warn(unused_assignments)]
#![warn(unused_variables)]

use crate::state;
use state::*;

pub struct ExState {
    count: f32,
}

impl ExState {
    pub fn new() -> Self {
        Self { count: 0f32 }
    }
}

impl State for ExState {
    fn input(&mut self, _dt: &f32) {
		
    }

    fn update(&mut self, dt: &f32) {
        self.count = self.count + 1f32;

        println!("The count is {} and the dt is {}.", self.count, dt);
    }
}
