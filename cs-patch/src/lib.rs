use da_interface::{Config, Midi, make_config, config, Param};

#[no_mangle] pub fn init(params: &mut Vec<Param>) -> Config { 
    make_config("DefaultCsound", 1, 0, 1, 1, 0);
    let csd = da_csound::get_csd_from_env();
    let csd = da_csound::parse_config(csd, params);
    da_csound::init2(csd, config());

    config().clone()
}

#[no_mangle] pub fn html() -> String { r#""#.to_string() }
#[no_mangle] pub fn css() -> String { r#""#.to_string() }
#[no_mangle] pub fn js() -> String { da_webui::create_js(r#"createDefaultUI();"#) }

#[no_mangle] pub fn next_block(samples: &Vec<Vec<f64>>, params: &Vec<Param>) { 
    da_csound::process_next_block(samples, params);
}

#[no_mangle] pub fn next(samples: &mut [f64; 32], time_in_samples: u64, midi_in: &Vec<Midi>, _midi_out: &mut Vec<Midi>, params: &mut Vec<Param>, done: &mut bool) {
    midi_in.iter().for_each(|m| {
        da_csound::midi::process_midi(m);
    });

    da_csound::process(time_in_samples, params, samples, done);
}