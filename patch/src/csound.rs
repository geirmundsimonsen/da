#![allow(unused_imports, unused_variables, dead_code)]
use da_interface::{Config, Midi, system_playback, keyboard_out, make_config, config, connect, self_midi_in, self_out, Param, lin_param, exp_param};

pub fn init(params: &mut Vec<Param>) -> Config {
    make_config("DefaultCsound", 1, 0, 1, 1, 0);
    let csd = da_csound::get_csd_from_env();
    let csd = da_csound::parse_config(csd, params);
    da_csound::init2(csd, config());

    config().clone()
}

pub fn html() -> String { r#""#.to_string() }
pub fn css() -> String { r#""#.to_string() }
pub fn js() -> String {
    da_webui::create_js(r#"createDefaultUI();"#)
}

pub fn next(samples: &mut [f64; 32], time_in_samples: u64, midi_in: &Vec<Midi>, _midi_out: &mut Vec<Midi>, params: &mut Vec<Param>, done: &mut bool) {
    midi_in.iter().for_each(|m| {
        if let Midi::On(on) = m {
            da_csound::send_instr_event(&vec![1.0, 0.0, 0.5, on.note as f64]);
        }
    });

    da_csound::process(time_in_samples, params, samples, done);
}