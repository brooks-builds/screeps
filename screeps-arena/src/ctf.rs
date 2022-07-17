use log::warn;
use screeps_arena::{
    game::utils::get_objects_by_prototype, prototypes, Creep, Flag, OwnedStructureProperties,
    ReturnCode, StructureTower,
};

use crate::global::{
    role::Role,
    utilities::{create_creeps_array, get_creeps, object_to_creep},
};

enum CreepType {
    Fighter,
    Ranger,
    Healer,
}

/// # Strategy
///
/// ## Turrets
///
/// - heal defender
/// - attack all enemies within the swamp near our flag
///
/// ## Creeps
///
/// ### Defender
///
/// Choose the first healer and assign it the role defender. It will be the one protecting the flag.
///
/// While enemies are on the other side of the map, collect powerups. Otherwise head back to base
/// and protect the flag.
///
/// ### Attackers
///
/// All attackers focus fire enemy creeps (starting with the one closest to our flag.)
///
/// Healers move with the pack, or heal the nearest creep with damage
///
/// ## All
///
/// Once all enemies are dead, swarm the flag
pub fn run(tick: u32) {
    let my_creeps = get_creeps(true);
    let my_flag = get_flag(true);
    let enemies = get_creeps(false);
    let my_towers = get_towers(true);

    if tick == 1 {
        let mut assigned_defender = false;

        for creep in &my_creeps {
            match determine_creep_type(creep) {
                CreepType::Healer => {
                    if !assigned_defender {
                        Role::Defender.attach_to_creep(creep);
                        assigned_defender = true;
                    } else {
                        Role::Attacker.attach_to_creep(creep);
                    }
                }
                _ => Role::Attacker.attach_to_creep(creep),
            }
        }
    }

    if let Some(closest_enemy) = my_flag.find_closest_by_range(&create_creeps_array(&enemies)) {
        let enemy_distance_to_flag = my_flag.get_range_to(&closest_enemy);
        let closest_enemy_creep = object_to_creep(&closest_enemy).unwrap();

        for creep in &my_creeps {
            let role = Role::from(creep);

            match role {
                Role::Defender => run_defender(
                    creep,
                    &my_flag,
                    &closest_enemy_creep,
                    enemy_distance_to_flag,
                ),
                Role::Attacker => run_attacker(
                    creep,
                    &closest_enemy_creep,
                    enemy_distance_to_flag,
                    &my_flag,
                    &my_creeps,
                ),
            }
        }
        run_towers(&my_towers, &closest_enemy_creep, enemy_distance_to_flag);
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
fn run_defender(creep: &Creep, flag: &Flag, closest_enemy: &Creep, enemy_distance: u8) {
    if enemy_distance <= 85 {
        creep.move_to(flag, None);

        warn!("Closest enemy is {enemy_distance} away from our flag");
        if enemy_distance <= 1 {
            creep.attack(closest_enemy);
        } else if enemy_distance <= 4 {
            creep.ranged_attack(closest_enemy);
        } else {
            creep.heal(&creep);
        }
    } else {
        creep.heal(&creep);
        let body_parts = get_objects_by_prototype(prototypes::BODY_PART);
        if let Some(body_part) = body_parts.first() {
            creep.move_to(body_part, None);
        }
    }
}

fn run_attacker(
    creep: &Creep,
    closest_enemy: &Creep,
    enemy_distance: u8,
    my_flag: &Flag,
    my_creeps: &Vec<Creep>,
) {
    if enemy_distance < 5 {
        let result = match determine_creep_type(creep) {
            CreepType::Fighter => Some((creep.attack(closest_enemy), closest_enemy)),
            CreepType::Ranger => Some((creep.ranged_attack(closest_enemy), closest_enemy)),
            CreepType::Healer => {
                if let Some(hurt_creep) = find_first_hurt_creep(my_creeps) {
                    Some((creep.heal(hurt_creep), hurt_creep))
                } else {
                    None
                }
            }
        };

        if let Some((result, target)) = result {
            if result == ReturnCode::NotInRange {
                creep.move_to(target, None);
            }
        }
    } else {
        creep.move_to(&my_flag, None);
    }
}

fn get_flag(my: bool) -> Flag {
    let flags = get_objects_by_prototype(prototypes::FLAG)
        .into_iter()
        .filter(|flag| flag.my().unwrap_or_default())
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

fn find_first_hurt_creep(creeps: &Vec<Creep>) -> Option<&Creep> {
    for creep in creeps {
        if creep.hits() < creep.hits_max() {
            return Some(creep);
        }
    }

    None
}
