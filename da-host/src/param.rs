use std::sync::Mutex;

pub static PARAMS: Mutex<Vec<f64>> = Mutex::new(vec![]);
