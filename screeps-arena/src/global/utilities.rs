use std::cell::Ref;

use js_sys::{Array, JsString, Object, Reflect};
use log::warn;
use screeps_arena::{
    game::utils::get_objects_by_prototype,
    prototypes::{self},
    Creep, StructureContainer,
};
use wasm_bindgen::JsValue;

#[allow(dead_code)]
pub fn get_creeps(my: bool) -> Vec<Creep> {
    get_objects_by_prototype(prototypes::CREEP)
        .into_iter()
        .filter(|creep| creep.my() == my)
        .collect()
}

#[allow(dead_code)]
pub fn creep_to_array(creeps: &Vec<Creep>) -> Array {
    let result = Array::new();

    for creep in creeps {
        result.push(creep);
    }

    result
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn get_closest_creep(creep: &Creep, other_creeps: &Vec<Creep>) -> Option<Creep> {
    let other_creeps_array = creep_to_array(other_creeps);
    if let Some(closest_creep_object) = creep.find_closest_by_path(&other_creeps_array, None) {
        object_to_creep(&closest_creep_object)
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn containers_to_array(containers: &Vec<StructureContainer>) -> Array {
    let array = Array::new();

    containers.iter().for_each(|object| {
        array.push(object);
    });

    array
}

pub fn create_position_object(x: u8, y: u8) -> Object {
    let position = Object::new();
    Reflect::set(
        &position,
        &JsValue::from_str("x"),
        &JsValue::from_str(x.to_string().as_str()),
    );
    Reflect::set(
        &position,
        &JsValue::from_str("y"),
        &JsValue::from_str(y.to_string().as_str()),
    );
    position
}
