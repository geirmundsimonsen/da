use da_interface::{Config, Midi, system_playback, keyboard_in, make_config, config, connect, self_midi_in, self_out};

#[no_mangle] pub fn init() -> Config {
    make_config("Test", 16, 0, 1, 1, 0);
    da_csound::init("test.csd", 64, config());

    connect(keyboard_in(), self_midi_in(1));
    connect(self_out(1), system_playback(9));

    config().clone()
}

#[no_mangle] pub fn html() -> String { r#""#.to_string() }
#[no_mangle] pub fn css() -> String { r#""#.to_string() }
#[no_mangle] pub fn js() -> String {
    da_webui::create_js(r#"

    createSlider("Freq", 0, 30, 120, 1);
    createSlider("Gain", 1, 1, 100, 1, "exp");
    createSlider("P.Dec.", 2, 0, 1, 3);
    createSlider("P.Str.", 3, 0, 10000, 0, "exp");
    
    "#)
}

#[no_mangle] pub fn next(samples: &mut [f64; 32], time_in_samples: u64, midi_in: &Vec<Midi>, _midi_out: &mut Vec<Midi>, params: &mut Vec<f64>) {
    if time_in_samples == 0 {
        params[0] = 50.0;
        params[1] = 2.0;
        params[2] = 0.3;
        params[3] = 100.0;
    }

    midi_in.iter().for_each(|m| {
        if let Midi::On(on) = m {
            da_csound::send_instr_event(&vec![1.0, 0.0, 0.5, on.note as f64]);
        }
    });

    da_csound::process(time_in_samples, params, samples);
}