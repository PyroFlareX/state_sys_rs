mod app;
use app::*;

mod ex_state;
use ex_state::*;

mod state;

fn main() {
    println!("Hello, world!");

    let mut app = App::new();
    app.push_state(Box::new(ExState::new()));
    app.run_loop();
}
