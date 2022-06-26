use screeps_arena::{game::utils::get_objects_by_prototype, prototypes, ReturnCode};

#[allow(dead_code)]
pub fn run() {
    let my_creep = get_objects_by_prototype(prototypes::CREEP)
        .into_iter()
        .find(|creep| creep.my())
        .unwrap();
    let enemy = get_objects_by_prototype(prototypes::CREEP)
        .into_iter()
        .find(|creep| !creep.my());

    if let Some(enemy) = enemy {
        if my_creep.attack(&enemy) == ReturnCode::NotInRange {
            my_creep.move_to(&enemy, None);
        }
    }
}
