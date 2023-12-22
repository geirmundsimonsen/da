use da_interface::{Config, Param};

static mut CSOUND: Option<csound::Csound> = None;
static mut KSMPS: u64 = 16;
static mut PARAM_UPDATE_SAMPLES: u64 = 48000;

pub fn init(csd: &str, ksmps: u64, param_update_hz: u32, config: &Config) {
    let csd = std::fs::read_to_string(csd).unwrap();
    let cs_instruments_end = csd.find("<CsInstruments>").unwrap() + 15;
    let csd = format!("{}\nsr={}\nksmps={}\nnchnls={}\n{}", &csd[..cs_instruments_end], 48000 * config.upsampling_factor, ksmps, config.num_out_channels, &csd[cs_instruments_end..]);
    

    unsafe {
        CSOUND = Some(csound::Csound::new());
        match CSOUND.as_ref().unwrap().compile_csd_text(csd) {
            Ok(_) => (),
            Err(e) => {
                panic!("Error compiling CSD: {}", e);
            }
        }
        match CSOUND.as_ref().unwrap().start() {
            Ok(_) => (),
            Err(e) => {
                panic!("Error starting Csound: {}", e);
            }
        }
        KSMPS = ksmps;
        PARAM_UPDATE_SAMPLES = (48000 * config.upsampling_factor / param_update_hz) as u64;
    }
}

pub fn process(time_in_samples: u64, params: &Vec<Param>, samples: &mut [f64; 32]) {
    unsafe {
        if time_in_samples % KSMPS as u64 == 0 {
            CSOUND.as_ref().unwrap().perform_ksmps();               
        }

        if time_in_samples % PARAM_UPDATE_SAMPLES == 0 {
            for i in 0..params.len() {
                CSOUND.as_mut().unwrap().set_control_channel(&format!("{}", params[i].name), params[i].value);
            }
        }
    
        let spout = CSOUND.as_ref().unwrap().get_output_buffer().unwrap();
        samples[0] = spout[time_in_samples as usize % 256];
    }
}

pub fn send_instr_event(pfields: &[f64]) {
    unsafe {
        CSOUND.as_ref().unwrap().send_score_event('i', pfields);
    }
}