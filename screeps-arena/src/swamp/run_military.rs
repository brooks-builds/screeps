use std::cell::Ref;

use eyre::{bail, Result};
use js_sys::{Array, Reflect};
use log::warn;
use screeps_arena::{game::utils::get_object_by_id, Creep, StructureSpawn};
use wasm_bindgen::JsValue;
use web_sys::console::warn;

use super::game_state::GameState;

pub fn run_military(
    creep: &Creep,
    enemies: &Vec<Creep>,
    enemy_spawn: &StructureSpawn,
    my_spawn: &StructureSpawn,
    _game_state: &GameState,
) -> Result<()> {
    if creep.hits() < creep.hits_max() {
        creep.heal(creep);
        creep.move_to(my_spawn, None);
        return Ok(());
    }

    if enemies.is_empty() {
        creep.move_to(&enemy_spawn, None);
        creep.ranged_attack(enemy_spawn);
        return Ok(());
    }

    let target_id = if let Some(target_id) = get_assigned_target(creep)? {
        target_id
    } else {
        let target_id = find_target(enemies, creep)?
            .ok_or(eyre::eyre!("Error getting enemy target id from gamestate"))?;

        target_id
    };

    if let Some(enemy) = get_enemy_by_id(target_id, enemies)? {
        creep.move_to(enemy, None);
        creep.ranged_attack(enemy);
    } else {
        unassign_target(creep)?;
    }

    Ok(())
}

fn get_assigned_target(creep: &Creep) -> Result<Option<f64>> {
    if let Ok(js_target) = Reflect::get(creep, &JsValue::from_str("target_id")) {
        if let Some(target_id) = js_target.as_f64() {
            Ok(Some(target_id))
        } else {
            Ok(None)
        }
    } else {
        bail!("Error getting target from creep");
    }
}

fn find_target(enemies: &Vec<Creep>, creep: &Creep) -> Result<Option<f64>> {
    let enemies_array = Array::new();
    for enemy in enemies {
        enemies_array.push(enemy);
    }

    if let Some(closest_creep) = creep.find_closest_by_path(&enemies_array, None) {
        if let Ok(js_id) = Reflect::get(&closest_creep, &JsValue::from_str("id")) {
            let id = js_id
                .as_string()
                .ok_or(eyre::eyre!("Error getting id"))?
                .parse::<f64>()?;
            Ok(Some(id))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn assign_target(creep: &Creep, target_id: f64) -> Result<()> {
    if let Err(_) = Reflect::set(
        creep,
        &JsValue::from_str("target_id"),
        &JsValue::from_f64(target_id),
    ) {
        bail!("Error setting target id on creep");
    }

    Ok(())
}

fn get_enemy_by_id(target_id: f64, enemies: &Vec<Creep>) -> Result<Option<&Creep>> {
    for enemy in enemies {
        let id = enemy
            .id()
            .as_string()
            .ok_or(eyre::eyre!("Error getting enemy id string"))?
            .parse::<f64>()?;
        if target_id == id {
            return Ok(Some(enemy));
        }
    }

    Ok(None)
}

fn unassign_target(creep: &Creep) -> Result<()> {
    if let Err(_) = Reflect::set(creep, &JsValue::from_str("target_id"), &JsValue::null()) {
        bail!("Error deleting target id from creep");
    }

    Ok(())
}
