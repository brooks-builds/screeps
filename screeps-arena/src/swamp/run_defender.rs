use js_sys::Object;
use screeps_arena::{Creep, StructureSpawn};

use crate::global::utilities::{create_position_object, creep_to_array, object_to_creep};

const THREAT_RANGE: u8 = 10;

pub fn run_defender(creep: &Creep, spawn: &StructureSpawn, enemies: &Vec<Creep>) {
    if let Some(enemy) = get_closest_enemy_within_threat_range(spawn, enemies) {
        creep.attack(&enemy);
        creep.ranged_attack(&enemy);
        creep.move_to(&enemy, None);
    } else {
        let staging_area = get_staging_area(spawn);
        creep.move_to(&staging_area, None);
    }
}

fn get_closest_enemy_within_threat_range(
    spawn: &StructureSpawn,
    enemies: &Vec<Creep>,
) -> Option<Creep> {
    if let Some(closest_enemy) = spawn.find_closest_by_path(&creep_to_array(enemies), None) {
        let distance_to_enemy = spawn.get_range_to(&closest_enemy);
        if distance_to_enemy <= THREAT_RANGE {
            object_to_creep(&closest_enemy)
        } else {
            None
        }
    } else {
        None
    }
}

fn get_staging_area(spawn: &StructureSpawn) -> Object {
    let y = spawn.y();
    let x = if spawn.x() < 20 {
        spawn.x() + 5
    } else {
        spawn.x() - 5
    };

    create_position_object(x, y)
}
