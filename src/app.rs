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

    pub fn pop_state(&mut self) -> Option<Box<dyn State>> {
        self.states.pop()
    }
}

#[cfg(test)]
mod test {
    use super::App;
    use super::State;

    //Make an example state
    pub struct TestState {
        count: f32,

        mode: i32,
    }

    impl TestState {
        pub fn new() -> Self {
            Self {
                count: 0f32,
                mode: 0,
            }
        }
    }

    impl State for TestState {
        fn input(&mut self, dt: &f32) {
            self.mode = 1;

            let _n = dt * 1.0f32 * 0.0f32 + 1.0f32;
        }

        fn update(&mut self, dt: &f32) {
            self.mode = 2;

            self.count = self.count + 1f32;

            println!("The count is {} and the dt is {}.", self.count, dt);
        }
    }

    #[test]
    fn basics() {
        let mut app = App::new();

        //Check empty and other internal states
        assert_eq!(app.states.len(), 0);

        app.push_state(Box::new(TestState::new()));
        //Check has added to list
        assert_eq!(app.states.len(), 1);

        //Check the end of popping for removal
        // assert_ne!(app.pop_state(), None);
        assert_eq!(app.states.len(), 0);

        //Add state back
        app.push_state(Box::new(TestState::new()));
        assert_eq!(app.states.len(), 1);

        //@TODO: Finish the tests

        //Check running
        //assert_eq!();

        // Check empty state behaves right
        // assert_eq!();

        /*
        // Populate States
        app.push_state(Box::new(TestState::new()));
        app.push_state(Box::new(TestState::new()));
        assert_eq!();

        // Check normal removal
        assert_eq!(app.pop_state();, Some(6));
        assert_eq!(app.pop_state();, Some(5));

        // Push some more just to make sure nothing's corrupted
        app.push_state();

        // Check normal removal
        assert_eq!();

        // Check exhaustion
        assert_eq!();
        */
    }
}
