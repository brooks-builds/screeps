use js_sys::{Array, JsString, Object, Reflect};
use log::warn;
use screeps_arena::{
    game::utils::get_objects_by_prototype,
    prototypes::{self, PrototypeConstant},
    Creep, GameObject, StructureContainer,
};
use wasm_bindgen::{JsObject, JsValue};

pub fn get_creeps(my: bool) -> Vec<Creep> {
    get_objects_by_prototype(prototypes::CREEP)
        .into_iter()
        .filter(|creep| creep.my() == my)
        .collect()
}

pub fn creep_to_array(creeps: &Vec<Creep>) -> Array {
    let result = Array::new();

    for creep in creeps {
        result.push(creep);
    }

    result
}

pub fn object_to_creep(object: &Object) -> Option<Creep> {
    let object_id = match Reflect::get(object, &JsValue::from_str("id")) {
        Ok(id) => JsString::from(id),
        Err(_) => {
            warn!("Error getting id from object");
            panic!();
        }
    };

    get_objects_by_prototype(prototypes::CREEP)
        .into_iter()
        .find(|creep| creep.id() == object_id)
}

pub fn object_to_container(object: &Object) -> Option<StructureContainer> {
    let object_id = match Reflect::get(object, &JsValue::from_str("id")) {
        Ok(id) => JsString::from(id),
        Err(_) => {
            warn!("Error getting id from object");
            panic!();
        }
    };

    get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER)
        .into_iter()
        .find(|structure| structure.id() == object_id)
}

pub fn get_closest_creep(creep: &Creep, other_creeps: &Vec<Creep>) -> Option<Creep> {
    let other_creeps_array = creep_to_array(other_creeps);
    if let Some(closest_creep_object) = creep.find_closest_by_path(&other_creeps_array, None) {
        object_to_creep(&closest_creep_object)
    } else {
        None
    }
}

pub fn containers_to_array(containers: &Vec<StructureContainer>) -> Array {
    let array = Array::new();

    containers.iter().for_each(|object| {
        array.push(object);
    });

    array
}
