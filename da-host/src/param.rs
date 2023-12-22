use std::sync::Mutex;

use da_interface::Param;

pub static PARAMS: Mutex<Vec<Param>> = Mutex::new(vec![]);
