use std::cell::RefMut;

use eyre::{bail, Result};
use screeps_arena::{Part, ResourceType};

use super::{error::CustomError, role::Role, state::State, Command};

#[derive(Default)]
pub struct SpawnerCommand {
    role: Role,
    body: Vec<Part>,
    cost: u32,
    energy_available: u32,
}

impl SpawnerCommand {
    pub fn new(role: Role) -> Self {
        Self {
            role,
            ..Default::default()
        }
    }

    fn update_cost(&mut self) {
        self.cost = self
            .body
            .iter()
            .map(|part| part.cost())
            .reduce(|total, part| total + part)
            .unwrap();
    }
}

impl Command for SpawnerCommand {
    fn execute(&mut self, state: RefMut<State>) -> Result<()> {
        match self.role {
            Role::Worker(carry_parts) => {
                let part = Part::Carry;
                self.body = vec![part; carry_parts];
                self.update_cost();
                self.energy_available = state
                    .my_spawn
                    .as_ref()
                    .unwrap()
                    .store()
                    .get_used_capacity(Some(ResourceType::Energy));

                if self.cost >= self.energy_available {
                    let needed = self.cost;
                    let had = self.energy_available;
                    Err(CustomError::NotEnoughEnergy { needed, had }.into())
                } else {
                    loop {
                        if self.cost + Part::Move.cost() <= self.energy_available {
                            self.body.push(Part::Move);
                            self.cost += Part::Move.cost();
                        } else {
                            break;
                        }
                    }
                    Ok(())
                }
            }
        }
    }
}
