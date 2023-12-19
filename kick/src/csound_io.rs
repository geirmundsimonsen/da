use da_interface::Config;

static mut CSOUND: Option<csound::Csound> = None;
static mut KSMPS: u32 = 16;

pub fn init(csd: &str, ksmps: u32, config: &Config) {
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
    }
}

pub fn process(time_in_samples: u64, params: &Vec<f64>, samples: &mut [f64; 32]) {
    unsafe {
        if time_in_samples % KSMPS as u64 == 0 {
            CSOUND.as_mut().unwrap().set_control_channel("param0", params[0]);
            CSOUND.as_mut().unwrap().set_control_channel("param1", params[1]);
            CSOUND.as_mut().unwrap().set_control_channel("param2", params[2]);
            CSOUND.as_mut().unwrap().set_control_channel("param3", params[3]);
            CSOUND.as_ref().unwrap().perform_ksmps();               
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