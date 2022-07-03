use js_sys::Reflect;
use log::warn;
use screeps_arena::{
    game::utils::{create_construction_site, get_objects_by_prototype},
    prototypes::{self, PrototypeConstant},
    ConstructionSite, Creep, HasStore, ReturnCode, StructureContainer,
};
use wasm_bindgen::JsValue;

enum Role {
    Collector(StructureContainer),
    Builder(Option<ConstructionSite>),
}

impl Role {
    pub fn new_role(creep: &Creep) -> Option<Self> {
        if creep.store().get_used_capacity(None) == 0 {
            Some(Self::Collector(get_container()))
        } else if creep.store().get_free_capacity(None) == 0 {
            Some(Self::Builder(get_construction_site()))
        } else {
            None
        }
    }
}

impl Into<JsValue> for Role {
    fn into(self) -> JsValue {
        match self {
            Role::Collector(_) => JsValue::from_str("Collector"),
            Role::Builder(_) => JsValue::from_str("Builder"),
        }
    }
}

impl From<&Creep> for Role {
    fn from(creep: &Creep) -> Self {
        if let Ok(role_string) = Reflect::get(creep, &JsValue::from_str("role")) {
            match js_value_to_string(role_string).as_str() {
                "Collector" => Self::Collector(get_container()),
                "Builder" => Self::Builder(get_construction_site()),
                _ => {
                    warn!("Error, attempted to create role from unknown string");
                    panic!();
                }
            }
        } else {
            warn!("Error getting role from creep object");
            panic!();
        }
    }
}

#[allow(dead_code)]
pub fn run(tick: u32) {
    if tick == 1 {
        let tower_position = (50, 55);

        create_construction_site(
            tower_position.0,
            tower_position.1,
            prototypes::STRUCTURE_TOWER.prototype(),
        )
        .unwrap();
    }

    for creep in get_objects_by_prototype(prototypes::CREEP) {
        if let Some(new_role) = Role::new_role(&creep) {
            store_role_on_creep(&creep, new_role);
        }

        match Role::from(&creep) {
            Role::Collector(container) => {
                let result = creep.withdraw(&container, screeps_arena::ResourceType::Energy, None);
                if result == ReturnCode::NotInRange {
                    creep.move_to(&container, None);
                }
            }
            Role::Builder(construction_site) => {
                if let Some(construction_site) = construction_site {
                    let result = creep.build(&construction_site);
                    if result == ReturnCode::NotInRange {
                        creep.move_to(&construction_site, None);
                    }
                }
            }
        }
    }
}

fn store_role_on_creep(creep: &Creep, role: Role) {
    if let Err(error) = Reflect::set(creep, &JsValue::from_str("role"), &role.into()) {
        warn!("Error setting role on Creep");
        panic!();
    }
}

fn js_value_to_string(js_value: JsValue) -> String {
    if !js_value.is_string() {
        warn!("JsValue is not a string");
        panic!();
    }

    js_value.as_string().unwrap()
}

fn get_container() -> StructureContainer {
    let containers = get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER);
    if containers.is_empty() {
        warn!("Could not find any containers");
        panic!();
    }

    containers[0].clone()
}

fn get_construction_site() -> Option<ConstructionSite> {
    let construction_sites = get_objects_by_prototype(prototypes::CONSTRUCTION_SITE);
    if construction_sites.is_empty() {
        None
    } else {
        Some(construction_sites[0].clone())
    }
}
