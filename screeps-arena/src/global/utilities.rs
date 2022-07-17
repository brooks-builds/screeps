use std::str::FromStr;

use js_sys::{Array, JsString, Object, Reflect};
use log::warn;
use screeps_arena::{
    game::utils::{get_object_by_id, get_objects_by_prototype},
    prototypes, Creep, GameObject,
};
use wasm_bindgen::JsValue;

pub fn get_creeps(my: bool) -> Vec<Creep> {
    get_objects_by_prototype(prototypes::CREEP)
        .into_iter()
        .filter(|creep| creep.my() == my)
        .collect()
}

pub fn create_creeps_array(creeps: &Vec<Creep>) -> Array {
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
