use crate::global::utilities::{create_position_object, creep_to_array};

use super::game_state::GameState;
use js_sys::Object;
use screeps_arena::{Creep, StructureSpawn};

const GROUP_SIZE_BEFORE_ATTACK: u8 = 8;
const THREAT_RANGE: u8 = 5;

pub fn run_attackers(
    creep: &Creep,
    game_state: &GameState,
    enemies: &[Creep],
    enemy_spawn: &StructureSpawn,
    my_spawn: &StructureSpawn,
) {
    if game_state.have_attackers >= GROUP_SIZE_BEFORE_ATTACK {
        let mut enemies_within_threat_range = get_enemies_within_threat_range(creep, enemies);

        if !enemies_within_threat_range.is_empty() {
            enemies_within_threat_range.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            let enemy = enemies_within_threat_range[0].1;
            creep.attack(enemy);
            creep.ranged_attack(enemy);
            creep.move_to(enemy, None);
        } else {
            creep.attack(enemy_spawn);
            creep.ranged_attack(enemy_spawn);
            creep.move_to(enemy_spawn, None);
        }
    } else {
        let staging_area = create_staging_area(my_spawn);
        creep.move_to(&staging_area, None);
    }
}

fn get_enemies_within_threat_range<'a>(
    creep: &Creep,
    enemies: &'a [Creep],
) -> Vec<(u8, &'a Creep)> {
    enemies
        .iter()
        .map(|enemy| {
            let enemy_position = create_position_object(enemy.x(), enemy.y());
            let range = creep.get_range_to(&enemy_position);
            (range, enemy)
        })
        .filter(|(range, enemy)| *range <= THREAT_RANGE)
        .collect()
}

fn create_staging_area(spawn: &StructureSpawn) -> Object {
    create_position_object(spawn.x(), spawn.y() - 5)
}
