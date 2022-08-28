use std::convert::TryFrom;

use eyre::{bail, Result};
use js_sys::Reflect;
use log::warn;
use screeps_arena::{Creep, StructureSpawn};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use super::role::EnemyRole;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub have_initial_collectors: u8,
    pub want_initial_collectors: u8,
    pub initial_collector_positions: [(u8, u8); 3],
    pub initial_collector_ids: [Option<f64>; 3],
    pub enemy_healer_ids: Vec<f64>,
    pub enemy_melee_ids: Vec<f64>,
    pub enemy_ranger_ids: Vec<f64>,
    pub enemy_collector_ids: Vec<f64>,
    pub enemy_unkown_ids: Vec<f64>,
}

impl GameState {
    pub fn new(spawn: &StructureSpawn) -> Result<Self> {
        let have_initial_collectors = 0;
        let want_initial_collectors = 3;
        let initial_collector_positions = Self::calculate_initial_collector_position(spawn);
        let initial_collector_ids = [None, None, None];
        let enemy_healer_ids = vec![];
        let enemy_melee_ids = vec![];
        let enemy_ranger_ids = vec![];
        let enemy_collector_ids = vec![];
        let enemy_unkown_ids = vec![];

        Ok(Self {
            have_initial_collectors,
            want_initial_collectors,
            initial_collector_positions,
            initial_collector_ids,
            enemy_healer_ids,
            enemy_collector_ids,
            enemy_melee_ids,
            enemy_ranger_ids,
            enemy_unkown_ids,
        })
    }

    pub fn save(&self, spawn: &StructureSpawn) -> Result<()> {
        let serialized_state = serde_json::to_string(self)?;
        if let Err(_error) = Reflect::set(
            spawn,
            &JsValue::from_str("game_state"),
            &JsValue::from_str(&serialized_state),
        ) {
            bail!("Error saving state to spawn");
        }

        Ok(())
    }

    pub fn load(spawn: &StructureSpawn) -> Result<Self> {
        match Reflect::get(spawn, &JsValue::from_str("game_state")) {
            Ok(value) => {
                let serialized_state = value
                    .as_string()
                    .ok_or(eyre::eyre!("Error converting state value to string"))?;
                let state = serde_json::from_str(&serialized_state)?;
                Ok(state)
            }
            Err(_) => {
                bail!("Error loading state");
            }
        }
    }

    pub fn have_all_initial_collectors(&self) -> bool {
        self.initial_collector_ids[0].is_some()
            && self.initial_collector_ids[1].is_some()
            && self.initial_collector_ids[2].is_some()
    }

    pub fn reset_enemy_ids(&mut self) {
        self.enemy_healer_ids = vec![];
        self.enemy_unkown_ids = vec![];
        self.enemy_collector_ids = vec![];
        self.enemy_melee_ids = vec![];
        self.enemy_ranger_ids = vec![];
    }

    pub fn store_enemy_ids(&mut self, enemies: &Vec<Creep>) -> Result<()> {
        for enemy in enemies {
            if let Some(enemy_id) = enemy.id().as_string() {
                let enemy_id = enemy_id.parse()?;
                let enemy_role = EnemyRole::try_from(enemy)?;
                match enemy_role {
                    EnemyRole::Healer => self.enemy_healer_ids.push(enemy_id),
                    EnemyRole::Unknown => self.enemy_unkown_ids.push(enemy_id),
                    EnemyRole::Collector => self.enemy_collector_ids.push(enemy_id),
                    EnemyRole::Melee => self.enemy_melee_ids.push(enemy_id),
                    EnemyRole::Ranger => self.enemy_ranger_ids.push(enemy_id),
                }
            }
        }
        Ok(())
    }

    fn calculate_initial_collector_position(spawn: &StructureSpawn) -> [(u8, u8); 3] {
        let y = spawn.y();
        let spawn_x = spawn.x();
        if spawn_x < 20 {
            [(spawn_x - 3, y), (spawn_x - 2, y), (spawn_x - 1, y)]
        } else {
            [(spawn_x + 3, y), (spawn_x + 2, y), (spawn_x + 1, y)]
        }
    }
}
