use std::cell::RefMut;

use eyre::Result;
use screeps_arena::game::utils::get_ticks;

use self::state::{State, STATE};

trait Command {
    fn execute(&mut self, state: RefMut<State>) -> Result<()>;
}

mod error;
mod role;
mod spawner;
pub mod state;

pub fn run() {
    STATE.with(|state| {
        let ticks = get_ticks();

        if ticks == 1 {
            state.borrow_mut().initialize();
        }
    });
}
