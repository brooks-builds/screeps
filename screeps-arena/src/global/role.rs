use std::thread::panicking;

use js_sys::Reflect;
use log::warn;
use screeps_arena::Creep;
use wasm_bindgen::JsValue;

pub enum Role {
    Defender,
    Attacker,
}

impl Role {
    pub fn attach_to_creep(self, creep: &Creep) {
        if let Err(error) = Reflect::set(creep, &JsValue::from_str("role"), &self.js_value()) {
            warn!("error attaching role to creep");
            panic!();
        }
    }

    fn js_value(&self) -> JsValue {
        JsValue::from_str(match self {
            Role::Defender => "defender",
            Role::Attacker => "attacker",
        })
    }
}

impl From<&Creep> for Role {
    fn from(creep: &Creep) -> Self {
        let role_string = match Reflect::get(creep, &JsValue::from_str("role")) {
            Ok(role) => role.as_string().unwrap(),
            Err(_) => {
                warn!("Error getting role from creep");
                panic!();
            }
        };

        match role_string.as_str() {
            "defender" => Self::Defender,
            "attacker" => Self::Attacker,
            _ => {
                warn!("unknown role");
                panic!();
            }
        }
    }
}
