// next will be to run the attackers. Each attacker is really made of 2 units, one is a giant creep with nothing but ranged attack. The other is a fast creep with nothing but move. The mover tows the shooter around
// attack all enemy creeps, then the enemy spawn
#![allow(dead_code)]

mod commands;
mod game_state;
mod role;
mod run_initial_collector;
mod run_military;
mod spawner;
mod state;

use std::{cell::RefCell, convert::TryFrom, rc::Rc};

use eyre::{bail, Result};
use screeps_arena::{
    game::{self, utils::get_objects_by_prototype},
    prototypes, Part, StructureContainer,
};

use crate::global::utilities::{get_creeps, get_spawn, Side};

use self::{
    commands::{Command, CreateCollectorCreepCommand},
    game_state::GameState,
    role::Role,
    run_military::run_military,
    state::{State, STATE},
};

pub fn run(ticks: u32) -> Result<()> {
    STATE.with(|state| {
        let state = state.clone();
        let mut queue = state
            .borrow_mut()
            .queue
            .drain(..)
            .collect::<Vec<Box<dyn Command>>>();
        if ticks == 1 {
            queue.push(Box::new(CreateCollectorCreepCommand));
            queue.push(Box::new(CreateCollectorCreepCommand));
            queue.push(Box::new(CreateCollectorCreepCommand));
        }

        let failed_jobs = process_queue(queue, state.clone());
        state.borrow_mut().queue = failed_jobs;
    });

    Ok(())

    // spawner::run_spawner(&my_spawn, &mut game_state)?;

    // for my_creep in &my_creeps {
    //     let creep_role = Role::try_from(my_creep)?;
    //     match creep_role {
    //         Role::None => bail!("trying to run a creep without a role"),
    //         Role::InitialCollector => {
    //             if !containers.is_empty() {
    //                 run_initial_collector::run_initial_collector(
    //                     my_creep,
    //                     &my_spawn,
    //                     &mut game_state,
    //                     &my_creeps,
    //                     &containers[0],
    //                 )?;
    //             }
    //         }
    //         Role::Military => {
    //             run_military(my_creep, &enemies, &enemy_spawn, &my_spawn, &game_state)?
    //         }
    //     }
    // }

    // game_state.save(&my_spawn)
}

fn process_queue(queue: Vec<Box<dyn Command>>, state: Rc<RefCell<State>>) -> Vec<Box<dyn Command>> {
    queue
        .into_iter()
        .filter_map(|command| {
            if command.execute(state.clone()) {
                None
            } else {
                Some(command.clone())
            }
        })
        .collect()
}
