use std::{cell::Ref, convert::TryFrom};

use eyre::bail;
use js_sys::Reflect;
use log::warn;
use screeps_arena::Creep;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
pub enum Role {
    None,
    InitialCollector,
    Military,
}

impl Role {
    pub fn add_to_creep(&self, creep: &Creep) -> eyre::Result<()> {
        let role_string = self.to_string();

        if let Err(_) = Reflect::set(
            creep,
            &JsValue::from_str("role"),
            &JsValue::from_str(&role_string),
        ) {
            bail!("Error setting role on creep");
        }

        Ok(())
    }
}

impl Default for Role {
    fn default() -> Self {
        Self::None
    }
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::None => "None",
            Role::InitialCollector => "InitialCollector",
            Role::Military => "Military",
        }
        .to_owned()
    }
}

impl TryFrom<&Creep> for Role {
    type Error = eyre::Error;

    fn try_from(creep: &Creep) -> Result<Self, Self::Error> {
        // if let Ok(js_role) = Reflect::get(creep, &JsValue::from_str("role")) {
        //     if let Some(role_string) = js_role.as_string() {
        //     } else {
        //         warn!("Could not get role string off of creep object");
        //         panic!();
        //     }
        // }
        if let Ok(js_role) = Reflect::get(creep, &JsValue::from_str("role")) {
            let role_string = js_role
                .as_string()
                .ok_or(eyre::eyre!("Error getting role string from creep object"))?;
            Ok(match role_string.as_str() {
                "None" => Self::None,
                "InitialCollector" => Self::InitialCollector,
                "Military" => Self::Military,
                _ => bail!("Unknown role"),
            })
        } else {
            bail!("Error getting the js value role from creep object");
        }
    }
    // fn try_from(creep: &Creep) -> Self {
    // }
}

pub enum EnemyRole {
    Healer,
    Unknown,
    Collector,
    Melee,
    Ranger,
}

impl TryFrom<&Creep> for EnemyRole {
    type Error = eyre::Error;

    fn try_from(creep: &Creep) -> Result<Self, Self::Error> {
        for body_part in creep.body().iter() {
            match body_part.part() {
                screeps_arena::Part::Carry => return Ok(Self::Collector),
                screeps_arena::Part::Attack => return Ok(Self::Melee),
                screeps_arena::Part::RangedAttack => return Ok(Self::Ranger),
                screeps_arena::Part::Heal => return Ok(Self::Healer),
                _ => continue,
            }
        }

        Ok(Self::Unknown)
    }
}
