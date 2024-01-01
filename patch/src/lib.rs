mod scratchpad;
mod bass;
mod kick;
mod csound;

use da_interface::{Config, Midi, Param};

use csound as current;

#[no_mangle] pub fn init(params: &mut Vec<Param>) -> Config { current::init(params) }
#[no_mangle] pub fn html() -> String { current::html() }
#[no_mangle] pub fn css() -> String { current::css() }
#[no_mangle] pub fn js() -> String { current::js() }
#[no_mangle] pub fn next(samples: &mut [f64; 32], time_in_samples: u64, midi_in: &Vec<Midi>, _midi_out: &mut Vec<Midi>, params: &mut Vec<Param>, done: &mut bool) {
    current::next(samples, time_in_samples, midi_in, _midi_out, params, done);
}