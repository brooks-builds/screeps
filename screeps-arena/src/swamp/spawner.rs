use super::{game_state::GameState, role::Role};
use eyre::{bail, Result};
use log::warn;
use screeps_arena::{Part, StructureSpawn};

const INITIAL_COLLECTOR_BODY: [Part; 2] = [Part::Move, Part::Carry];
const MILITARY_BODY: [Part; 14] = [
    Part::Move,
    Part::Move,
    Part::Move,
    Part::Move,
    Part::Move,
    Part::Move,
    Part::Move,
    Part::Move,
    Part::Move,
    Part::Move,
    Part::Move,
    Part::Move,
    Part::RangedAttack,
    Part::Heal,
];

pub fn run_spawner(spawn: &StructureSpawn, game_state: &mut GameState) -> Result<()> {
    if game_state.have_initial_collectors < game_state.want_initial_collectors {
        spawn_creep(
            &INITIAL_COLLECTOR_BODY,
            spawn,
            Role::InitialCollector,
            game_state,
        )
    } else {
        spawn_creep(&MILITARY_BODY, spawn, Role::Military, game_state)
    }
}

fn spawn_creep(
    body: &[Part],
    spawn: &StructureSpawn,
    role: Role,
    game_state: &mut GameState,
) -> Result<()> {
    if let Ok(creep) = spawn.spawn_creep(body) {
        match role {
            Role::None => bail!("Attempting to spawn creep with No Role"),
            Role::InitialCollector => game_state.have_initial_collectors += 1,
            Role::Military => warn!("Spawning Military unit"),
        }
        role.add_to_creep(&creep)?;
    }
    Ok(())
}
