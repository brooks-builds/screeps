use js_sys::Reflect;
use log::warn;
use screeps_arena::{
    game::utils::get_objects_by_prototype,
    prototypes::{self},
    Creep, Part, ReturnCode, StructureSpawn,
};
use std::{collections::HashMap, hash::Hash};
use wasm_bindgen::JsValue;

const DESIRED_SPAWN_REFILLER_COUNT: u8 = 2;
const DESIRED_FIGHTER_COUNT: u8 = 1;
const DESIRED_RANGER_COUNT: u8 = 5;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
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

impl Into<JsValue> for Role {
    fn into(self) -> JsValue {
        JsValue::from_str(match self {
            Role::SpawnRefiller => "SpawnRefiller",
            Role::Builder => "Builder",
            Role::TurretRefiller => "TurretRefiller",
            Role::Fighter => "Fighter",
            Role::Ranger => "Ranger",
            Role::Healer => "Healer",
            Role::None => "None",
        })
    }
}

enum CreepState {
    Work,
    Harvest,
    Unknown,
}

impl From<&Creep> for CreepState {
    fn from(creep: &Creep) -> Self {
        match get_custom_string_from_creep(creep, "state").as_str() {
            "Work" => Self::Work,
            "Harvest" => Self::Harvest,
            _ => Self::Unknown,
        }
    }
}

impl Into<JsValue> for CreepState {
    fn into(self) -> JsValue {
        JsValue::from_str(match self {
            CreepState::Work => "Work",
            CreepState::Harvest => "Harvest",
            CreepState::Unknown => "Unknown",
        })
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
/// State
///     Work
///     Harvest
#[allow(dead_code)]
pub fn run(tick: u32) {
    let spawn = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN)[0].clone();

    if tick == 1 {
        // create_tower_construction_sites(&spawn);
    }

    let my_creeps: Vec<Creep> = get_creeps(true);
    let enemy_creeps = get_creeps(false);

    let mut role_count = count_roles(&my_creeps);

    if let Some(role_created) = control_spawner(&spawn, &role_count) {
        increment_role_count(role_created, &mut role_count);
    }

    for creep in &my_creeps {
        assign_creep_state(creep);
        match Role::from(creep) {
            Role::SpawnRefiller => run_spawn_refiller_role(creep, &spawn),
            Role::Builder => run_builder(creep),
            Role::TurretRefiller => todo!(),
            Role::Fighter => run_fighter(creep, &enemy_creeps),
            Role::Ranger => run_ranger(creep, &enemy_creeps),
            Role::Healer => todo!(),
            Role::None => {}
        }
    }

    if tick % 25 == 0 {
        log_role_counts(&role_count)
    }
}

fn get_creeps(is_my: bool) -> Vec<Creep> {
    get_objects_by_prototype(prototypes::CREEP)
        .into_iter()
        .filter(|creep| creep.my() == is_my)
        .collect()
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
                String::new()
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

fn control_spawner(spawn: &StructureSpawn, role_count: &HashMap<Role, u8>) -> Option<Role> {
    if get_creep_count(role_count, Role::SpawnRefiller) < DESIRED_SPAWN_REFILLER_COUNT {
        return spawn_creep(spawn, Role::SpawnRefiller);
    }

    if get_creep_count(role_count, Role::Ranger) < DESIRED_RANGER_COUNT {
        return spawn_creep(spawn, Role::Ranger);
    }

    if get_creep_count(role_count, Role::Fighter) < DESIRED_FIGHTER_COUNT {
        return spawn_creep(spawn, Role::Fighter);
    }

    None
}

fn get_creep_count(role_count: &HashMap<Role, u8>, role: Role) -> u8 {
    role_count.get(&role).map_or(0, |count| *count)
}

fn attach_role_to_creep(role: Role, creep: &Creep) {
    if let Err(error) = Reflect::set(creep, &JsValue::from_str("role"), &role.into()) {
        warn!(
            "Error setting role {:?} on creep {}: {:?}",
            role,
            creep.id(),
            error
        );
        panic!();
    }
}

fn attach_to_creep(value: JsValue, creep: &Creep, key: &str) {
    warn!("attaching {key} to creep");
    if let Err(_error) = Reflect::set(creep, &JsValue::from_str(key), &value) {
        warn!("Error setting {key} {:?} on creep {}", value, creep.id());
        panic!();
    }
}

fn increment_role_count(role: Role, role_count: &mut HashMap<Role, u8>) {
    let count = role_count.entry(role).or_insert(0);
    *count += 1;
}

fn spawn_creep(spawn: &StructureSpawn, role: Role) -> Option<Role> {
    let body = create_creep_body(role);
    if let Ok(creep) = spawn.spawn_creep(&body) {
        attach_role_to_creep(role, &creep);
        Some(role)
    } else {
        None
    }
}

fn create_creep_body(role: Role) -> Vec<Part> {
    match role {
        Role::SpawnRefiller => vec![Part::Carry, Part::Work, Part::Move, Part::Move],
        Role::Builder => vec![Part::Carry, Part::Work, Part::Move, Part::Move],
        Role::TurretRefiller => vec![Part::Carry, Part::Work, Part::Move, Part::Move],
        Role::Fighter => vec![Part::Attack, Part::Tough, Part::Move, Part::Move],
        Role::Ranger => vec![Part::RangedAttack, Part::Tough, Part::Move, Part::Move],
        Role::Healer => vec![Part::Heal, Part::Move],
        Role::None => {
            warn!("Attempting to build creep body with no role");
            panic!();
        }
    }
}

fn run_spawn_refiller_role(creep: &Creep, spawn: &StructureSpawn) {
    let state = CreepState::from(creep);

    match state {
        CreepState::Work => {
            let result = creep.transfer(spawn, screeps_arena::ResourceType::Energy, None);
            if result == ReturnCode::NotInRange {
                creep.move_to(spawn, None);
            }
        }
        CreepState::Harvest => harvest(creep),
        CreepState::Unknown => {
            warn!("Spawn Refiller in unknown state!");
            panic!();
        }
    }
}

fn assign_creep_state(creep: &Creep) {
    let state = CreepState::from(creep);
    let role = Role::from(creep);

    match role {
        Role::SpawnRefiller | Role::Builder => match state {
            CreepState::Work => {
                if creep.store().get_used_capacity(None) == 0 {
                    attach_to_creep(CreepState::Harvest.into(), creep, "state");
                }
            }
            CreepState::Harvest => {
                if creep.store().get_free_capacity(None) == 0 {
                    attach_to_creep(CreepState::Work.into(), creep, "state");
                }
            }
            CreepState::Unknown => attach_to_creep(CreepState::Harvest.into(), creep, "state"),
        },
        Role::Fighter => match state {
            CreepState::Work => {}
            _ => attach_to_creep(CreepState::Work.into(), creep, "state"),
        },
        _ => {}
    }
}

fn harvest(creep: &Creep) {
    let sources = get_objects_by_prototype(prototypes::SOURCE);
    if sources.is_empty() {
        warn!("Could not find any energy sources");
        panic!();
    }

    let result = creep.harvest(&sources[0]);
    if result == ReturnCode::NotInRange {
        creep.move_to(&sources[0], None);
    }
}

fn run_builder(creep: &Creep) {
    let state = CreepState::from(creep);
    match state {
        CreepState::Work => {
            let construction_sites = get_objects_by_prototype(prototypes::CONSTRUCTION_SITE);
            if construction_sites.is_empty() {
                return;
            }

            let result = creep.build(&construction_sites[0]);
            if result == ReturnCode::NotInRange {
                creep.move_to(&construction_sites[0], None);
            }
        }
        CreepState::Harvest => harvest(creep),
        CreepState::Unknown => {}
    }
}

fn run_fighter(creep: &Creep, enemies: &Vec<Creep>) {
    let enemy_healers = enemies
        .iter()
        .filter(|creep| is_healer(creep))
        .collect::<Vec<&Creep>>();

    let target = if !enemy_healers.is_empty() {
        enemy_healers[0]
    } else if !enemies.is_empty() {
        &enemies[0]
    } else {
        return;
    };

    if creep.attack(target) == ReturnCode::NotInRange {
        creep.move_to(&target, None);
    }
}

fn is_healer(creep: &Creep) -> bool {
    for body_part in creep.body() {
        if body_part.part() == Part::Heal {
            return true;
        }
    }

    false
}

fn run_ranger(creep: &Creep, enemies: &Vec<Creep>) {
    let enemy_healers = enemies
        .iter()
        .filter(|creep| is_healer(creep))
        .collect::<Vec<&Creep>>();

    let target = if !enemy_healers.is_empty() {
        enemy_healers[0]
    } else if !enemies.is_empty() {
        &enemies[0]
    } else {
        return;
    };

    if creep.ranged_attack(target) == ReturnCode::NotInRange {
        creep.move_to(&target, None);
    }
}
