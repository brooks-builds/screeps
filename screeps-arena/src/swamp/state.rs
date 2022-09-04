use std::{cell::RefCell, rc::Rc};

use screeps_arena::{
    game::utils::get_objects_by_prototype, prototypes, Creep, OwnedStructureProperties,
    StructureContainer, StructureSpawn,
};

use super::commands::Command;

thread_local! {
    pub static STATE: Rc<RefCell<State>> = Rc::new(RefCell::new(State::default()));
}

pub struct State {
    pub my_spawn: StructureSpawn,
    pub enemy_spawn: StructureSpawn,
    pub my_side: Side,
    pub spawn_containers: Vec<StructureContainer>,
    pub initial_collectors: Vec<Creep>,
    pub queue: Vec<Box<dyn Command>>,
}

impl Default for State {
    fn default() -> Self {
        let mut my_spawn = None;
        let mut enemy_spawn = None;

        get_objects_by_prototype(prototypes::STRUCTURE_SPAWN)
            .into_iter()
            .for_each(|spawn| {
                if spawn.my().unwrap_or_default() {
                    my_spawn = Some(spawn.clone());
                } else {
                    enemy_spawn = Some(spawn.clone());
                }
            });

        let my_spawn_x = my_spawn.as_ref().unwrap().x();

        let my_side = if my_spawn_x < 50 {
            Side::Left
        } else {
            Side::Right
        };

        let containers = get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER)
            .into_iter()
            .filter(|container| container.store().get_used_capacity(None) > 0)
            .filter(move |container| match my_side {
                Side::Left => container.x() == my_spawn_x - 4,
                Side::Right => container.x() == my_spawn_x + 4,
            })
            .collect::<Vec<StructureContainer>>();

        Self {
            my_spawn: my_spawn.unwrap(),
            enemy_spawn: enemy_spawn.unwrap(),
            my_side,
            spawn_containers: containers,
            initial_collectors: vec![],
            queue: vec![],
        }
    }
}

#[derive(Clone, Copy)]
pub enum Side {
    Left,
    Right,
}
