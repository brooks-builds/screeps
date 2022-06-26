use screeps_arena::{game::utils::get_objects_by_prototype, prototypes};

#[allow(dead_code)]
pub fn run() {
    let creeps = get_objects_by_prototype(prototypes::CREEP);
    let flags = get_objects_by_prototype(prototypes::FLAG);

    creeps[0].move_to(&flags[0], None);
}
