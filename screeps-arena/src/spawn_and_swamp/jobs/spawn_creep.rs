use screeps_arena::{Part, Resource, ResourceType};

use super::Job;

pub struct SpawnCreepJob {
    my_type: CreepType,
    body: Vec<Part>,
    cost: u32,
}

impl SpawnCreepJob {
    pub fn new(my_type: CreepType) -> Self {
        let body = match &my_type {
            CreepType::InitialCollector => vec![Part::Carry, Part::Move, Part::Move],
        };
        let cost = body
            .iter()
            .map(|part| part.cost())
            .reduce(|total, cost| total + cost)
            .unwrap();

        Self {
            my_type,
            body,
            cost,
        }
    }
}

impl Job for SpawnCreepJob {
    fn run(
        &self,
        wrapped_state: std::rc::Rc<std::cell::RefCell<crate::spawn_and_swamp::state::State>>,
    ) -> Option<Box<dyn Job>> {
        let mut state = wrapped_state.borrow_mut();
        if let None = state.creep_spawning {
            if self.cost
                > state
                    .my_spawn
                    .store()
                    .get_used_capacity(Some(ResourceType::Energy))
            {
                if let Ok(creep) = state.my_spawn.spawn_creep(&self.body) {
                    state.creep_spawning = Some(creep);
                    None
                } else {
                    Some(Box::new(SpawnCreepJob::new(self.my_type)))
                }
            } else {
                Some(Box::new(SpawnCreepJob::new(self.my_type)))
            }
        } else {
            Some(Box::new(SpawnCreepJob::new(self.my_type)))
        }
    }
}

#[derive(Clone, Copy)]
pub enum CreepType {
    InitialCollector,
}
