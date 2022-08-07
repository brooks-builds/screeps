use js_sys::Reflect;
use log::warn;
use screeps_arena::{Creep, Part};
use wasm_bindgen::JsValue;

#[derive(PartialEq)]
pub enum Role {
    Defender,
    Attacker,
    Healer,
    Collector,
    Unknown,
}

#[allow(dead_code)]
impl Role {
    pub fn attach_to_creep(self, creep: &Creep) {
        if let Err(error) = Reflect::set(creep, &JsValue::from_str("role"), &self.js_value()) {
            warn!("error attaching role to creep: {:?}", error);
            panic!();
        }
    }

    pub fn create_body_for_role(&self) -> Vec<Part> {
        match self {
            Role::Defender => todo!(),
            Role::Attacker => vec![
                Part::Tough,
                Part::Tough,
                Part::Tough,
                Part::Tough,
                Part::Tough,
                Part::RangedAttack,
                Part::RangedAttack,
                Part::RangedAttack,
                Part::Move,
                Part::Move,
                Part::Move,
                Part::Move,
                Part::Move,
                Part::Heal,
            ],
            Role::Healer => todo!(),
            Role::Collector => vec![
                Part::Tough,
                Part::Tough,
                Part::Tough,
                Part::Tough,
                Part::Tough,
                Part::Carry,
                Part::Carry,
                Part::Carry,
                Part::Move,
                Part::Move,
                Part::Move,
                Part::Move,
                Part::Move,
                Part::Move,
            ],
            Role::Unknown => todo!(),
        }
    }

    fn js_value(&self) -> JsValue {
        JsValue::from_str(match self {
            Role::Defender => "defender",
            Role::Attacker => "attacker",
            Role::Healer => "healer",
            Role::Collector => "collector",
            Role::Unknown => "unknown",
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
            "healer" => Self::Healer,
            "collector" => Self::Collector,
            _ => Self::Unknown,
        }
    }
}
