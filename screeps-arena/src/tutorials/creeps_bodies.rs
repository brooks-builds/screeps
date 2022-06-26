use screeps_arena::{game::utils::get_objects_by_prototype, prototypes, Creep, ReturnCode};

#[allow(dead_code)]
pub fn run() {
    let mut fighter = None;
    let mut ranger = None;
    let mut healer = None;
    let mut enemy = None;

    get_objects_by_prototype(prototypes::CREEP)
        .into_iter()
        .for_each(|creep| {
            if !creep.my() {
                enemy = Some(creep);
                return;
            }

            match get_creep_type(&creep) {
                CreepType::Fighter => fighter = Some(creep),
                CreepType::Ranger => ranger = Some(creep),
                CreepType::Healer => healer = Some(creep),
            }
        });

    let enemy = if let Some(enemy) = enemy {
        enemy
    } else {
        return;
    };

    if let Some(fighter) = &fighter {
        if fighter.attack(&enemy) == ReturnCode::NotInRange {
            fighter.move_to(&enemy, None);
        }
    }

    if let Some(ranger) = &ranger {
        if ranger.ranged_attack(&enemy) == ReturnCode::NotInRange {
            ranger.move_to(&enemy, None);
        }
    }

    if let Some(healer) = &healer {
        if let Some(fighter) = &fighter {
            if has_been_hurt(fighter) {
                heal_or_move(&healer, fighter);
            }
        }

        if let Some(ranger) = &ranger {
            if has_been_hurt(ranger) {
                heal_or_move(&healer, ranger);
            }
        }

        if has_been_hurt(&healer) {
            healer.heal(&healer);
        }
    }
}

enum CreepType {
    Fighter,
    Ranger,
    Healer,
}

fn get_creep_type(creep: &Creep) -> CreepType {
    for body_part in creep.body() {
        match body_part.part() {
            screeps_arena::Part::Attack => return CreepType::Fighter,
            screeps_arena::Part::RangedAttack => return CreepType::Ranger,
            screeps_arena::Part::Heal => return CreepType::Healer,
            _ => continue,
        }
    }

    unreachable!()
}

fn has_been_hurt(creep: &Creep) -> bool {
    creep.hits() < creep.hits_max()
}

fn heal_or_move(healer: &Creep, target: &Creep) {
    if healer.heal(target) == ReturnCode::NotInRange {
        healer.move_to(target, None);
    }
}
