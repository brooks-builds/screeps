use super::game_state::GameState;
use crate::global::role::Role;
use screeps_arena::StructureSpawn;

const DEFAULT_ROLE: Role = Role::Attacker;

pub fn run_spawner(spawn: &StructureSpawn, game_state: &GameState) {
    let needed_role = determine_needed_role(game_state);
    let body = needed_role.create_body_for_role();
    if let Ok(creep) = spawn.spawn_creep(&body) {
        needed_role.attach_to_creep(&creep);
    }
}

fn determine_needed_role(game_state: &GameState) -> Role {
    if game_state.have_collectors < game_state.min_collectors {
        Role::Collector
    } else if game_state.have_defenders < game_state.min_defenders {
        Role::Defender
    } else if game_state.have_healers < game_state.min_healers {
        Role::Healer
    } else {
        DEFAULT_ROLE
    }
}
