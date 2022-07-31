use js_sys::Array;
use log::warn;
use screeps_arena::{
    game::utils::get_objects_by_prototype, prototypes, BodyPart, Creep, Flag,
    OwnedStructureProperties, ReturnCode, StructureTower,
};

use crate::global::{
    role::Role,
    utilities::{create_creeps_array, get_closest_creep, get_creeps, object_to_creep},
};

#[derive(PartialEq)]
enum CreepType {
    Fighter,
    Ranger,
    Healer,
}

#[derive(Clone, Copy)]
enum EnemyState {
    EnemySide,
    River,
    OurSide,
    MostlyDead,
    Turtling,
}

impl EnemyState {
    pub fn new(enemies: &Vec<Creep>, distance_to_our_flag: u8, tick: u32) -> Self {
        if enemies.len() <= 1 {
            Self::MostlyDead
        } else if tick >= 1500 {
            Self::Turtling
        } else {
            if distance_to_our_flag < 5 {
                Self::OurSide
            } else if distance_to_our_flag < 75 {
                Self::River
            } else {
                Self::EnemySide
            }
        }
    }
}

/// # Strategy
///
/// ## Turrets
///
/// - [x] attack all enemies within the swamp near our flag
///
/// ## Creeps
///
/// ### Defender
///
/// - [x] Choose the first healer and assign it the role defender.
/// - [x] It will move to the flag, sit on it, and heal itself if damaged
///
/// ### Attackers
///
/// - go out and collect body parts to power up
/// - return to middle of our side of map
/// - if the opponents are moving onto our side
///     - attack the nearest enemy
///
/// - When not healing, healers stay near the flag
/// - At 1500 ticks, if no enemies on our side, take the opponents flag
///
/// ## All
///
/// Once all enemies are dead, swarm the flag
pub fn run(tick: u32) {
    #[cfg(feature = "arena-capture-the-flag")]
    {
        let my_creeps = get_creeps(true);
        let my_flag = get_flag(true);
        let enemies = get_creeps(false);
        let my_towers = get_towers(true);
        let enemy_flag = get_flag(false);

        if tick == 1 {
            let mut assigned_defender = false;

            for creep in &my_creeps {
                match determine_creep_type(creep) {
                    CreepType::Healer => {
                        if !assigned_defender {
                            Role::Defender.attach_to_creep(creep);
                            assigned_defender = true;
                        } else {
                            Role::Healer.attach_to_creep(creep);
                        }
                    }
                    _ => Role::Attacker.attach_to_creep(creep),
                }
            }
        }

        if let Some(closest_enemy) = my_flag.find_closest_by_range(&create_creeps_array(&enemies)) {
            let enemy_distance_to_flag = my_flag.get_range_to(&closest_enemy);
            let closest_enemy_creep = object_to_creep(&closest_enemy).unwrap();
            let enemy_state = EnemyState::new(&enemies, enemy_distance_to_flag, tick);

            for creep in &my_creeps {
                let role = Role::from(creep);

                match role {
                    Role::Defender => run_defender(creep, &my_flag),
                    Role::Attacker | Role::Healer => run_attacker(
                        creep,
                        &my_flag,
                        &my_creeps,
                        enemy_state,
                        &enemies,
                        &enemy_flag,
                    ),
                    // Role::Healer => run_healer(creep, &my_creeps, &my_flag),
                }
            }
            run_towers(&my_towers, &closest_enemy_creep, enemy_distance_to_flag);
        } else {
            for creep in &my_creeps {
                creep.move_to(&enemy_flag, None);
            }
        }
    }
}

fn determine_creep_type(creep: &Creep) -> CreepType {
    for body_part in creep.body() {
        match body_part.part() {
            screeps_arena::Part::Attack => return CreepType::Fighter,
            screeps_arena::Part::RangedAttack => return CreepType::Ranger,
            screeps_arena::Part::Heal => return CreepType::Healer,
            _ => continue,
        }
    }

    warn!("Could not determine creep type");
    panic!();
}

/// If there are any enemies on our side of the map
///
/// - goto the flag
/// - melee attack
/// - heal
/// - ranged attack
///
/// Otherwise collect body parts
fn run_defender(creep: &Creep, flag: &Flag) {
    creep.move_to(flag, None);
    creep.heal(creep);
}

fn run_attacker(
    creep: &Creep,
    my_flag: &Flag,
    my_creeps: &Vec<Creep>,
    enemy_state: EnemyState,
    enemy_creeps: &Vec<Creep>,
    enemy_flag: &Flag,
) {
    let hurt_creeps = get_hurt_creeps(my_creeps);
    if !hurt_creeps.is_empty() && Role::from(creep) == Role::Healer {
        if let Some(closest_hurt_creep) = get_closest_creep(creep, &hurt_creeps) {
            if creep.heal(&closest_hurt_creep) == ReturnCode::NotInRange {
                creep.ranged_heal(&closest_hurt_creep);
                creep.move_to(&closest_hurt_creep, None);
            }
            return;
        }
    }

    match enemy_state {
        EnemyState::EnemySide => {
            let body_parts = get_objects_by_prototype(prototypes::BODY_PART);
            if body_parts.is_empty() {
                creep.move_to(my_flag, None);
            } else {
                let body_parts_array = create_body_parts_array(&body_parts);
                if let Some(closest_body_part) = creep.find_closest_by_path(&body_parts_array, None)
                {
                    creep.move_to(&closest_body_part, None);
                }
            }
        }
        EnemyState::River => {
            creep.move_to(my_flag, None);
        }
        EnemyState::OurSide | EnemyState::Turtling => {
            if let Some(closest_enemy) = get_closest_creep(creep, enemy_creeps) {
                creep.ranged_attack(&closest_enemy);
                creep.attack(&closest_enemy);
                creep.move_to(&closest_enemy, None);
            }
        }
        EnemyState::MostlyDead => {
            if let Some(enemy) = enemy_creeps.first() {
                creep.ranged_attack(enemy);
                creep.attack(enemy);
                creep.move_to(enemy, None);
            } else {
                creep.move_to(&enemy_flag, None);
            }
        }
    }
}

fn get_flag(my: bool) -> Flag {
    let flags = get_objects_by_prototype(prototypes::FLAG)
        .into_iter()
        .filter(|flag| flag.my().unwrap_or_default() == my)
        .collect::<Vec<Flag>>();

    flags[0].clone()
}

fn get_towers(my: bool) -> Vec<StructureTower> {
    get_objects_by_prototype(prototypes::STRUCTURE_TOWER)
        .into_iter()
        .filter(|tower| tower.my().unwrap_or_default() == my)
        .collect()
}

fn run_towers(towers: &Vec<StructureTower>, closest_enemy: &Creep, enemy_distance: u8) {
    if enemy_distance <= 5 {
        for tower in towers {
            tower.attack(closest_enemy);
        }
    }
}

fn get_hurt_creeps(creeps: &Vec<Creep>) -> Vec<Creep> {
    creeps
        .clone()
        .into_iter()
        .filter(|creep| creep.hits_max() > creep.hits())
        .collect()
}

fn create_body_parts_array(body_parts: &Vec<BodyPart>) -> Array {
    let body_parts_array = Array::new();

    for body_part in body_parts {
        body_parts_array.push(body_part);
    }

    body_parts_array
}
