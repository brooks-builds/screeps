use screeps_arena::{Creep, ReturnCode, StructureSpawn};

use crate::global::utilities::{containers_to_array, get_containers, object_to_container};

pub fn run_collector(creep: &Creep, spawn: &StructureSpawn) {
    if have_energy(creep) {
        if creep.transfer(spawn, screeps_arena::ResourceType::Energy, None)
            == ReturnCode::NotInRange
        {
            creep.move_to(spawn, None);
        }

        return;
    }

    let containers = get_containers(true);
    if let Some(closest_container) =
        creep.find_closest_by_path(&containers_to_array(&containers), None)
    {
        let container = object_to_container(&closest_container).unwrap();
        if creep.withdraw(&container, screeps_arena::ResourceType::Energy, None)
            == ReturnCode::NotInRange
        {
            creep.move_to(&container, None);
        }
    } else {
        creep.move_to(spawn, None);
    }
}

fn have_energy(creep: &Creep) -> bool {
    creep.store().get_used_capacity(None) > 0
}
