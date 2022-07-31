use eyre::{bail, Result};
use log::warn;
use screeps_arena::{
    game::utils::get_objects_by_prototype, prototypes, BodyPart, Creep, OwnedStructureProperties,
    Part, ResourceType, ReturnCode, StructureContainer, StructureSpawn,
};

use crate::global::{
    role::Role,
    utilities::{
        containers_to_array, creep_to_array, get_creeps, object_to_container, object_to_creep,
    },
};

struct CreepTypes {
    pub have_collectors: u8,
    pub want_collectors: u8,
    pub have_attackers: u8,
    pub want_attackers: u8,
}

impl CreepTypes {
    pub fn init(creeps: &Vec<Creep>) -> Self {
        let mut creep_types = Self::default();
        creep_types.want_collectors = 5;

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
            want_collectors: 5,
            have_attackers: 0,
            want_attackers: 100,
        }
    }
}

pub fn run(tick: u32) -> Result<()> {
    let my_spawn = get_spawn(true);
    let my_creeps = get_creeps(true);
    let creep_types = CreepTypes::init(&my_creeps);
    let enemy_creeps = get_creeps(false);
    let enemy_spawn = get_spawn(false);

    if let Some(needed_role) = creep_types.needed_role() {
        spawn_creep(&my_spawn, needed_role);
    }

    my_creeps.iter().for_each(|creep| {
        let role = Role::from(creep);
        match role {
            Role::Collector => run_collector(creep, &my_spawn),
            Role::Attacker => run_attacker(creep, &enemy_creeps, &enemy_spawn),
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

fn run_attacker(creep: &Creep, enemies: &Vec<Creep>, enemy_spawn: &StructureSpawn) {
    let enemies_array = creep_to_array(enemies);
    creep.heal(creep);
    if let Some(closest_enemy_object) = creep.find_closest_by_path(&enemies_array, None) {
        let enemy = object_to_creep(&closest_enemy_object).unwrap();
        if creep.ranged_attack(&enemy) == ReturnCode::NotInRange {
            creep.move_to(&enemy, None);
        }
    } else if creep.ranged_attack(enemy_spawn) == ReturnCode::NotInRange {
        creep.move_to(enemy_spawn, None);
    }
}
