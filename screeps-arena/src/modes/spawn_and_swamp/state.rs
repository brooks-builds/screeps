use std::{cell::RefCell, rc::Rc};

use screeps_arena::{
    game::utils::get_objects_by_prototype, prototypes, OwnedStructureProperties, StructureSpawn,
};

use super::Command;

#[derive(Default)]
pub struct State {
    pub my_spawn: Option<StructureSpawn>,
    pub enemy_spawn: Option<StructureSpawn>,
    pub commands: Vec<Box<dyn Command>>,
}

impl State {
    pub fn initialize(&mut self) {
        get_objects_by_prototype(prototypes::STRUCTURE_SPAWN)
            .into_iter()
            .for_each(|spawn| {
                if let Some(is_mine) = spawn.my() {
                    if is_mine {
                        self.my_spawn = Some(spawn.clone());
                    } else {
                        self.enemy_spawn = Some(spawn.clone());
                    }
                }
            })
    }
}

thread_local! {
    pub static STATE: Rc<RefCell<State>> = Rc::new(RefCell::new(State::default()));
}
