#![allow(unused_imports, unused_variables, dead_code)]
use da_interface::{Config, Midi, Param, system_playback, reaper_in, keyboard_in, make_config, config, connect, self_midi_in, self_out, ParamType, list_param, exp_param};

pub fn init(params: &mut Vec<Param>) -> Config {
    make_config("Bass", 16, 0, 1, 1, 0);
    da_csound::init("src/bass.csd", 64, 100, config());

    connect(keyboard_in(), self_midi_in(1));
    connect(self_out(1), system_playback(9));
    //connect(self_out(1), reaper_in(1));

    params.push(exp_param("Detune", 0.05, 0.0, 12.0, 2));
    params.push(exp_param("F.Freq", 555.0, 20.0, 10000.0, 0));
    params.push(exp_param("F.Q", 0.1, 0.0, 1.5, 2));
    params.push(list_param("F.Type", 8.0, vec!["lowpass2 (99, v4.0)", "lpf18 (00, v4.10)'", "moogladder (05, v5)", "mvclpf3 (16, v6.07)", "spf (21)", "skf (21)", "svn (21)", "zdf ladder (17)", "k35 (17)"]));

    config().clone()
}

pub fn html() -> String { r#""#.to_string() }
pub fn css() -> String { r#""#.to_string() }
pub fn js() -> String { da_webui::create_js(r#"createDefaultUI();"#) }

pub fn next(samples: &mut [f64; 32], time_in_samples: u64, midi_in: &Vec<Midi>, _midi_out: &mut Vec<Midi>, params: &mut Vec<Param>, done: &mut bool) {   
    midi_in.iter().for_each(|m| {
        if let Midi::On(on) = m {
            da_csound::send_instr_event(&vec![1.0, 0.0, -1.0, on.note as f64]);
        }
    });

    da_csound::process(time_in_samples, params, samples, done);
}