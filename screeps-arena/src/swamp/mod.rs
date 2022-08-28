// next will be to run the attackers. Each attacker is really made of 2 units, one is a giant creep with nothing but ranged attack. The other is a fast creep with nothing but move. The mover tows the shooter around
// attack all enemy creeps, then the enemy spawn
#![allow(dead_code)]

mod game_state;
mod role;
mod run_initial_collector;
mod run_military;
mod spawner;

use std::convert::TryFrom;

use eyre::{bail, Result};
use screeps_arena::{
    game::{self, utils::get_objects_by_prototype},
    prototypes, StructureContainer,
};

use crate::global::utilities::{get_creeps, get_spawn, Side};

use self::{game_state::GameState, role::Role, run_military::run_military};

pub fn run(ticks: u32) -> Result<()> {
    let my_spawn = get_spawn(true).ok_or(eyre::eyre!("Error getting my spawn"))?;
    let my_side = if my_spawn.x() < 20 {
        Side::Left
    } else {
        Side::Right
    };
    let my_creeps = get_creeps(true);
    let enemies = get_creeps(false);
    let enemy_spawn = get_spawn(false).ok_or(eyre::eyre!("Error getting enemy spawn"))?;
    let containers = get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER)
        .into_iter()
        .filter(|container| container.store().get_used_capacity(None) > 0)
        .filter(|container| match my_side {
            Side::Left => container.x() == my_spawn.x() - 4,
            Side::Right => container.x() == my_spawn.x() + 4,
        })
        .collect::<Vec<StructureContainer>>();

    if ticks == 1 {
        let empty_gamestate = GameState::new(&my_spawn)?;
        empty_gamestate.save(&my_spawn)?;
    }

    let mut game_state = GameState::load(&my_spawn)?;
    game_state.reset_enemy_ids();
    game_state.store_enemy_ids(&enemies)?;
    spawner::run_spawner(&my_spawn, &mut game_state)?;

    for my_creep in &my_creeps {
        let creep_role = Role::try_from(my_creep)?;
        match creep_role {
            Role::None => bail!("trying to run a creep without a role"),
            Role::InitialCollector => {
                if !containers.is_empty() {
                    run_initial_collector::run_initial_collector(
                        my_creep,
                        &my_spawn,
                        &mut game_state,
                        &my_creeps,
                        &containers[0],
                    )?;
                }
            }
            Role::Military => {
                run_military(my_creep, &enemies, &enemy_spawn, &my_spawn, &game_state)?
            }
        }
    }

    game_state.save(&my_spawn)
}
