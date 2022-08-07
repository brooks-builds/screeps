use eyre::Result;
use log::warn;
use screeps_arena::{
    game::utils::{get_object_by_id, get_objects_by_prototype},
    prototypes, Creep, OwnedStructureProperties, ResourceType, ReturnCode, StructureContainer,
    StructureSpawn,
};

use crate::global::{
    role::Role,
    utilities::{
        containers_to_array, create_position_object, creep_to_array, get_creeps,
        object_to_container, object_to_creep,
    },
};

#[allow(dead_code)]
struct CreepTypes {
    pub have_collectors: u8,
    pub want_collectors: u8,
    pub have_attackers: u8,
    pub want_attackers: u8,
}

#[allow(dead_code)]
impl CreepTypes {
    pub fn init(creeps: &Vec<Creep>) -> Self {
        let mut creep_types = Self::default();

        creeps.iter().for_each(|creep| {
            let role = Role::from(creep);
            match role {
                Role::Collector => creep_types.have_collectors += 1,
                Role::Attacker => creep_types.have_attackers += 1,
                _ => (),
            }
        });

        creep_types
    }

    pub fn needed_role(&self) -> Option<Role> {
        if self.have_collectors < self.want_collectors {
            Some(Role::Collector)
        } else if self.have_attackers < self.want_attackers {
            Some(Role::Attacker)
        } else {
            None
        }
    }
}

impl Default for CreepTypes {
    fn default() -> Self {
        Self {
            have_collectors: 0,
            want_collectors: 1,
            have_attackers: 0,
            want_attackers: 100,
        }
    }
}

#[allow(dead_code)]
pub fn run(_tick: u32) -> Result<()> {
    let my_spawn = get_spawn(true);
    let my_creeps = get_creeps(true);
    let creep_types = CreepTypes::init(&my_creeps);
    let enemy_creeps = get_creeps(false);
    let enemy_spawn = get_spawn(false);
    let closest_enemy_to_spawn = get_closest_enemy_to_spawn(&my_spawn, &enemy_creeps);
    let my_attackers = filter_creeps_by_role(&my_creeps, Role::Attacker);

    if let Some(needed_role) = creep_types.needed_role() {
        spawn_creep(&my_spawn, needed_role);
    }

    my_creeps.iter().for_each(|creep| {
        let role = Role::from(creep);
        match role {
            Role::Collector => run_collector(creep, &my_spawn),
            Role::Attacker => run_attacker(
                creep,
                &enemy_spawn,
                &my_attackers,
                &my_spawn,
                closest_enemy_to_spawn.as_ref(),
            ),
            _ => (),
        }
    });

    Ok(())
}

fn get_spawn(my: bool) -> StructureSpawn {
    let spawns = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN)
        .into_iter()
        .filter(|spawn| spawn.my().unwrap() == my)
        .collect::<Vec<StructureSpawn>>();

    spawns[0].clone()
}

fn spawn_creep(spawn: &StructureSpawn, role: Role) {
    if let Ok(new_creep) = spawn.spawn_creep(&role.create_body_for_role()) {
        role.attach_to_creep(&new_creep);
    }
}

fn run_collector(creep: &Creep, spawn: &StructureSpawn) {
    if creep.store().get_used_capacity(None) == 0 {
        let containers = get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER)
            .into_iter()
            .filter(|container| container.store().get_used_capacity(None) > 0)
            .collect::<Vec<StructureContainer>>();
        let containers_array = containers_to_array(&containers);
        if let Some(closest_container_object) = creep.find_closest_by_path(&containers_array, None)
        {
            let closest_container = object_to_container(&closest_container_object).unwrap();
            if creep.withdraw(&closest_container, ResourceType::Energy, None)
                == ReturnCode::NotInRange
            {
                creep.move_to(&closest_container, None);
            }
        }
    } else {
        if creep.transfer(spawn, ResourceType::Energy, None) == ReturnCode::NotInRange {
            creep.move_to(spawn, None);
        }
    }
}

fn run_attacker(
    creep: &Creep,
    enemy_spawn: &StructureSpawn,
    other_attackers: &Vec<&Creep>,
    my_spawn: &StructureSpawn,
    closest_enemy_to_spawn: Option<&Creep>,
) {
    let my_creeps_distance_to_spawn = calculate_distance_from_spawn(other_attackers, my_spawn);
    if should_creeps_regroup(&my_creeps_distance_to_spawn) {
        let shortest_index = get_index_of_shortest_distance(&my_creeps_distance_to_spawn);
        creep.move_to(&other_attackers[shortest_index], None);
        creep.ranged_mass_attack();
    } else {
        if let Some(enemy) = closest_enemy_to_spawn {
            if creep.ranged_attack(enemy) == ReturnCode::NotInRange {
                creep.ranged_mass_attack();
            }
            creep.move_to(&enemy, None);
        } else {
            creep.move_to(enemy_spawn, None);
            creep.ranged_attack(enemy_spawn);
        }
    }
    creep.heal(creep);
}

fn calculate_distance_from_spawn(creeps: &Vec<&Creep>, spawn: &StructureSpawn) -> Vec<u8> {
    creeps
        .iter()
        .map(|creep| creep.get_range_to(&create_position_object(spawn.x(), spawn.y())))
        .collect()
}

fn should_creeps_regroup(distances_to_spawn: &Vec<u8>) -> bool {
    if distances_to_spawn.len() < 2 {
        return false;
    }

    let max_desired_distance_from_eachother = 4;
    let mut distances = distances_to_spawn.clone();
    distances.sort();

    distances[1] - distances[0] >= max_desired_distance_from_eachother
}

fn get_index_of_shortest_distance(distances_to_spawn: &Vec<u8>) -> usize {
    let distance = distances_to_spawn
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.cmp(b.1));
    distance.unwrap().0
}

fn get_closest_enemy_to_spawn(spawn: &StructureSpawn, enemies: &Vec<Creep>) -> Option<Creep> {
    let enemies_array = creep_to_array(enemies);
    let closest = spawn.find_closest_by_path(&enemies_array, None)?;

    object_to_creep(&closest)
}

fn filter_creeps_by_role(creeps: &Vec<Creep>, role: Role) -> Vec<&Creep> {
    creeps
        .iter()
        .filter(|creep| Role::from(*creep) == role)
        .collect()
}
