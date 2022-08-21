use crate::global::utilities::{create_position_object, creep_to_array, object_to_creep};
use js_sys::Object;
use screeps_arena::{Creep, ReturnCode, StructureSpawn};

pub fn run_healer(creep: &Creep, friends: &[Creep], spawn: &StructureSpawn) {
    if is_damaged(creep) {
        creep.heal(creep);
    } else {
        let damaged_friends = get_damaged_friends(friends);
        if let Some(closest_damaged_friend) =
            creep.find_closest_by_path(&creep_to_array(&damaged_friends), None)
        {
            let closest_damaged_friend = object_to_creep(&closest_damaged_friend).unwrap();
            if creep.heal(&closest_damaged_friend) == ReturnCode::NotInRange {
                creep.ranged_heal(&closest_damaged_friend);
                creep.move_to(&closest_damaged_friend, None);
            }
        } else {
            let staging_area = get_staging_area(spawn);
            creep.move_to(&staging_area, None);
        }
    }
}

fn is_damaged(creep: &Creep) -> bool {
    creep.hits() < creep.hits_max()
}

fn get_damaged_friends(creeps: &[Creep]) -> Vec<Creep> {
    creeps
        .iter()
        .filter(|creep| is_damaged(creep))
        .map(|creep| creep.clone())
        .collect()
}

fn get_staging_area(spawn: &StructureSpawn) -> Object {
    let x = if spawn.x() < 20 {
        spawn.x() + 3
    } else {
        spawn.x() - 3
    };

    create_position_object(x, spawn.y())
}
