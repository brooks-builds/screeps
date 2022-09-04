use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use screeps_arena::Part;

use super::state::State;

pub trait Command {
    fn execute(&self, state: Rc<RefCell<State>>) -> bool;
    fn clone(&self) -> Box<dyn Command>;
}

pub struct CreateCollectorCreepCommand;

impl Command for CreateCollectorCreepCommand {
    fn execute(&self, state: Rc<RefCell<State>>) -> bool {
        let body = [Part::Carry, Part::Move, Part::Move];
        let creep = if let Ok(creep) = state.borrow().my_spawn.spawn_creep(&body) {
            creep
        } else {
            return false;
        };
        state.borrow_mut().initial_collectors.push(creep);
        true
    }

    fn clone(&self) -> Box<dyn Command> {
        Box::new(CreateCollectorCreepCommand)
    }
}
