mod game_state;
mod run_attackers;
mod run_collector;
mod run_defender;
mod run_healer;
mod spawner;

use eyre::Result;
use log::warn;
use screeps_arena::{
    game::{self, utils::get_objects_by_prototype},
    prototypes,
};

use crate::global::{
    role::Role,
    utilities::{get_creeps, get_spawn},
};

use self::{
    game_state::GameState, run_attackers::run_attackers, run_collector::run_collector,
    run_defender::run_defender, run_healer::run_healer, spawner::run_spawner,
};

#[allow(dead_code)]
pub fn run(_tick: u32) -> Result<()> {
    let my_creeps = get_creeps(true);
    let enemies = get_creeps(false);
    let my_spawn = get_spawn(true).unwrap();
    let enemy_spawn = get_spawn(false).unwrap();
    let game_state = GameState::init(&my_creeps);

    run_spawner(&my_spawn, &game_state);

    my_creeps.iter().for_each(|creep| {
        let role = Role::from(creep);

        match role {
            Role::Defender => run_defender(creep, &my_spawn, &enemies),
            Role::Attacker => run_attackers(creep, &game_state, &enemies, &enemy_spawn, &my_spawn),
            Role::Healer => run_healer(creep, &my_creeps, &my_spawn),
            Role::Collector => run_collector(creep, &my_spawn),
            Role::Unknown => todo!(),
        }
    });

    Ok(())
}
