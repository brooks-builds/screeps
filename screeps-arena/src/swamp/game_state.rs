use screeps_arena::{game, Creep};

use crate::global::role::Role;

pub struct GameState {
    pub have_collectors: u8,
    pub min_collectors: u8,
    pub have_defenders: u8,
    pub min_defenders: u8,
    pub have_healers: u8,
    pub min_healers: u8,
    pub have_attackers: u8,
    pub min_attackers: u8,
}

impl GameState {
    pub fn init(creeps: &Vec<Creep>) -> Self {
        let mut game_state = Self::default();

        creeps.iter().for_each(|creep| {
            let role = Role::from(creep);
            match role {
                Role::Defender => game_state.have_defenders += 1,
                Role::Attacker => game_state.have_attackers += 1,
                Role::Healer => game_state.have_healers += 1,
                Role::Collector => game_state.have_collectors += 1,
                Role::Unknown => todo!(),
            }
        });

        game_state
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            min_collectors: 1,
            have_collectors: 0,
            have_defenders: 0,
            min_defenders: 1,
            have_healers: 0,
            min_healers: 1,
            have_attackers: 0,
            min_attackers: 3,
        }
    }
}
