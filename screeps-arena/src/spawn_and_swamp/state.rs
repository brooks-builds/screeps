use screeps_arena::{
    game::utils::get_objects_by_prototype, prototypes, Creep, OwnedStructureProperties,
    StructureSpawn,
};

use super::jobs::JobQueue;

pub struct State {
    pub job_queue: JobQueue,
    pub my_spawn: StructureSpawn,
    pub creep_spawning: Option<Creep>,
}

impl State {
    pub fn new() -> Self {
        let my_spawn = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN)
            .into_iter()
            .filter(|spawn| spawn.my().unwrap_or_default())
            .collect::<Vec<StructureSpawn>>()[0]
            .clone();

        Self {
            job_queue: JobQueue::new(),
            my_spawn,
            creep_spawning: None,
        }
    }
}
