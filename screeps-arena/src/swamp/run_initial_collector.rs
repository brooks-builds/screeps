use eyre::{bail, Result};
use screeps_arena::{Creep, ResourceType, StructureContainer, StructureSpawn};

use crate::global::utilities::{create_position_object, get_creep_id};

use super::game_state::GameState;

pub fn run_initial_collector(
    creep: &Creep,
    spawn: &StructureSpawn,
    game_state: &mut GameState,
    my_creeps: &Vec<Creep>,
    container: &StructureContainer,
) -> Result<()> {
    let id = get_creep_id(creep)?;
    let collector_index = get_collector_index(game_state, id)?;
    let (x, y) = game_state.initial_collector_positions[collector_index];
    let position = create_position_object(x, y);
    creep.move_to(&position, None);

    if !game_state.have_all_initial_collectors() {
        return Ok(());
    }

    match collector_index {
        2 => {
            creep.transfer(spawn, ResourceType::Energy, None);
        }
        1 => transfer_to_next_creep(&game_state, 2, my_creeps, creep)?,
        0 => {
            creep.withdraw(container, ResourceType::Energy, None);
            transfer_to_next_creep(&game_state, 1, my_creeps, creep)?;
        }
        _ => bail!("collector index is not what we expected: {collector_index}"),
    }
    Ok(())
}

fn get_collector_index(game_state: &mut GameState, creep_id: f64) -> Result<usize> {
    for (index, set_collector_id) in game_state.initial_collector_ids.iter_mut().enumerate() {
        match set_collector_id {
            Some(set_id) => {
                if *set_id == creep_id {
                    return Ok(index);
                } else {
                    continue;
                }
            }
            None => {
                *set_collector_id = Some(creep_id);
                return Ok(index);
            }
        };
    }

    bail!("Error, didn't find the positions for initial creep collectors");
}

fn transfer_to_next_creep(
    game_state: &GameState,
    index: usize,
    my_creeps: &Vec<Creep>,
    creep: &Creep,
) -> Result<()> {
    let next_creep_id = game_state.initial_collector_ids[index]
        .ok_or(eyre::eyre!("could not find intial collector"))?;

    let next_creep = my_creeps
        .iter()
        .find(|creep| get_creep_id(creep).unwrap() == next_creep_id)
        .ok_or(eyre::eyre!("Could not find next creep in line"))?;

    creep.transfer(next_creep, ResourceType::Energy, None);

    Ok(())
}
