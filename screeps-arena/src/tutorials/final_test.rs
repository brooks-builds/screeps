use std::{collections::HashMap, hash::Hash};

use js_sys::Reflect;
use log::warn;
use screeps_arena::{
    game::utils::{create_construction_site, get_objects_by_prototype},
    prototypes::{self, PrototypeConstant},
    Creep, StructureSpawn,
};
use wasm_bindgen::JsValue;
use web_sys::console::count;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Role {
    SpawnRefiller,
    Builder,
    TurretRefiller,
    Fighter,
    Ranger,
    Healer,
    None,
}

impl From<&Creep> for Role {
    fn from(creep: &Creep) -> Self {
        match get_custom_string_from_creep(creep, "role").as_str() {
            "SpawnRefiller" => Self::SpawnRefiller,
            "Builder" => Self::Builder,
            "TurretRefiller" => Self::TurretRefiller,
            "Fighter" => Self::Fighter,
            "Ranger" => Self::Ranger,
            "Healer" => Self::Healer,
            _ => Self::None,
        }
    }
}

/// # Strategy
///
/// Roles
///     SpawnRefiller
///     Builder
///     TurretRefiller
///     Fighter
///     Ranger
///     Healer
///
/// Jobs
///     Refill
///     Build
///     Harvest
///     Heal
///     RangedAttack
///     MeleeAttack
#[allow(dead_code)]
pub fn run(tick: u32) {
    let spawn = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN)[0].clone();

    if tick == 1 {
        create_tower_construction_sites(&spawn);
    }

    let creeps: Vec<Creep> = get_objects_by_prototype(prototypes::CREEP)
        .into_iter()
        .filter(|creep| creep.my())
        .collect();

    let role_count = count_roles(&creeps);
    if tick % 25 == 0 {
        log_role_counts(&role_count)
    }
}

fn create_tower_construction_sites(spawn: &StructureSpawn) {
    create_tower_construction_site(spawn.x() + 5, spawn.y());
    create_tower_construction_site(spawn.x(), spawn.y() - 5);
    create_tower_construction_site(spawn.x() - 5, spawn.y());
    create_tower_construction_site(spawn.x(), spawn.y() + 5);
}

fn create_tower_construction_site(x: u8, y: u8) {
    if let Err(error) = create_construction_site(x, y, prototypes::STRUCTURE_TOWER.prototype()) {
        warn!("Error creating construction site: {:?}", error)
    }
}

fn count_roles(creeps: &Vec<Creep>) -> HashMap<Role, u8> {
    let mut counts = HashMap::new();

    for creep in creeps {
        let role = Role::from(creep);

        let counts_role = counts.entry(role).or_insert(0);
        *counts_role += 1;
    }

    counts
}

fn get_custom_string_from_creep(creep: &Creep, key: &str) -> String {
    match Reflect::get(creep, &JsValue::from_str(key)) {
        Ok(value) => {
            if value.is_string() {
                value.as_string().unwrap()
            } else {
                warn!("Error, custom value with key {key} on creep is not a string");
                panic!();
            }
        }
        Err(error) => {
            warn!(
                "Error getting {key} from creep: {}",
                error.as_string().unwrap_or_default()
            );
            panic!();
        }
    }
}

fn log_role_counts(role_count: &HashMap<Role, u8>) {
    warn!("Creep Counts:");
    warn!("-----");
    for (role, count) in role_count {
        warn!("role: {:?} - {count}", role);
    }
    warn!("-----");
}
