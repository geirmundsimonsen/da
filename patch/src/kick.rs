#![allow(unused_imports, unused_variables, dead_code)]
use da_interface::{Config, Midi, system_playback, keyboard_out, make_config, config, connect, self_midi_in, self_out, Param, lin_param, exp_param};

pub fn init(params: &mut Vec<Param>) -> Config {
    make_config("Kick", 16, 0, 1, 1, 0);
    da_csound::init("src/kick.csd", 64, config());

    connect(keyboard_out(), self_midi_in(1));
    connect(self_out(1), system_playback(9));

    params.push(lin_param("Freq", 50.0, 30.0, 120.0, 1));
    params.push(exp_param("Gain", 2.0, 1.0, 100.0, 1));
    params.push(lin_param("P.Dec.", 0.3, 0.0, 1.0, 3));
    params.push(exp_param("P.Str.", 100.0, 0.0, 10000.0, 0));

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