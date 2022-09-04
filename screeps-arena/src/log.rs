use std::fmt::Debug;

use crate::js::log_string;

pub fn log_object<T: Debug>(label: &str, object: &T) {
    let string = format!("{label}: {:?}", object);
    log_string(&string);
}
