#![allow(unused_imports, dead_code)]
use da_interface::{Config, Midi, Param, system_playback, reaper_in, keyboard_in, make_config, config, connect, self_midi_in, self_out, ParamType, list_param, exp_param};

pub fn init(params: &mut Vec<Param>) -> Config {
    make_config("a01", 16, 0, 1, 1, 0);
    da_csound::init("src/scratchpad/a01.csd", 64, 100, config());

    connect(keyboard_in(), self_midi_in(1));
    connect(self_out(1), system_playback(9));

    config().clone()
}

pub fn html() -> String { r#""#.to_string() }
pub fn css() -> String { r#""#.to_string() }
pub fn js() -> String { da_webui::create_js(r#"createDefaultUI();"#) }

pub fn next(samples: &mut [f64; 32], time_in_samples: u64, midi_in: &Vec<Midi>, _midi_out: &mut Vec<Midi>, params: &mut Vec<Param>) {   
    da_csound::process(time_in_samples, params, samples);
}