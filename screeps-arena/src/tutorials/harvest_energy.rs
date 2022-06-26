use js_sys::Reflect;
use log::warn;
use screeps_arena::{
    game::utils::get_objects_by_prototype, prototypes, Creep, ReturnCode, Source, StructureSpawn,
};
use wasm_bindgen::JsValue;

struct Data {
    pub creep: Creep,
    pub spawn: StructureSpawn,
    pub energy_source: Source,
}

impl Data {
    pub fn init() -> Self {
        let creeps = get_objects_by_prototype(prototypes::CREEP);
        let spawns = get_objects_by_prototype(prototypes::STRUCTURE_SPAWN);
        let energy_sources = get_objects_by_prototype(prototypes::SOURCE);

        Self {
            creep: creeps[0].clone(),
            spawn: spawns[0].clone(),
            energy_source: energy_sources[0].clone(),
        }
    }

    pub fn assign_role(&self) {
        if let Some(new_role) = self.change_role() {
            assign_role_to_creep(&self.creep, new_role);
        }
    }

    fn change_role(&self) -> Option<Role> {
        match get_role_from_creep(&self.creep) {
            Role::Harvest => {
                if self.creep.store().get_free_capacity(None) == 0 {
                    Some(Role::Deliver)
                } else {
                    None
                }
            }
            Role::Deliver => {
                if self.creep.store().get_used_capacity(None) == 0 {
                    Some(Role::Harvest)
                } else {
                    None
                }
            }
            Role::None => Some(Role::Harvest),
        }
    }
}

enum Role {
    Harvest,
    Deliver,
    None,
}

impl From<String> for Role {
    fn from(role_string: String) -> Self {
        match role_string.as_str() {
            "harvest" => Self::Harvest,
            "deliver" => Self::Deliver,
            _ => Self::None,
        }
    }
}

impl Into<JsValue> for Role {
    fn into(self) -> JsValue {
        JsValue::from_str(match self {
            Role::Harvest => "harvest",
            Role::Deliver => "deliver",
            Role::None => "none",
        })
    }
}

#[allow(dead_code)]
pub fn run() {
    let data = Data::init();
    data.assign_role();
    match get_role_from_creep(&data.creep) {
        Role::Harvest => {
            if data.creep.harvest(&data.energy_source) == ReturnCode::NotInRange {
                data.creep.move_to(&data.energy_source, None);
            }
        }
        Role::Deliver => {
            if data
                .creep
                .transfer(&data.spawn, screeps_arena::ResourceType::Energy, None)
                == ReturnCode::NotInRange
            {
                data.creep.move_to(&data.spawn, None);
            }
        }
        Role::None => {}
    }
}

fn get_role_from_creep(creep: &Creep) -> Role {
    if let Ok(creep_role) = Reflect::get(creep, &js_role_key()) {
        if let Some(role_string) = creep_role.as_string() {
            Role::from(role_string)
        } else {
            Role::None
        }
    } else {
        Role::None
    }
}

fn assign_role_to_creep(creep: &Creep, role: Role) {
    if let Err(_) = Reflect::set(creep, &js_role_key(), &role.into()) {
        warn!("Error assigning role to creep");
        panic!();
    }
}

fn js_role_key() -> JsValue {
    JsValue::from_str("role")
}
