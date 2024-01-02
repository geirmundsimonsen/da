pub mod config;
pub mod midi;

use core::panic;

use da_interface::{CONFIG, Config, Param, ParamType, connect, keyboard_out, self_out, system_playback, self_midi_in, self_midi_out, reaper_midi_out, reaper_midi_in, system_capture, reaper_out, self_in, reaper_in};
use regex::Regex;

static mut CSOUND: Option<csound::Csound> = None;
static mut KSMPS: u64 = 16;
static mut IN_CH: u32 = 0;
static mut OUT_CH: u32 = 1;
static BUFFER_SIZE: u32 = 256;
static mut UPSAMPLING_FACTOR: u32 = 1;

pub fn get_csd_from_env() -> String {
    let csd_path = match std::env::var("DA_CSOUND") {
        Ok(csd) => csd,
        Err(_) => panic!("\nDA_CSOUND environment variable not set.\n")
    };

    println!("Using CSD: {}", csd_path);

    std::fs::read_to_string(csd_path).expect("\nError reading CSD file.\n")
}

pub fn parse_config(csd: String, params: &mut Vec<Param>) -> String {
    let config_start = csd.find("<Config>").expect("\n<Config> block missing.\n");
    let config_end = csd.find("</Config>").expect("\n</Config> terminator missing\n");

    let config_yml = &csd[config_start+9..config_end];
    
    let config = config::parse_config(config_yml);

    unsafe { CONFIG.as_mut().unwrap().name = config.name.unwrap_or("DA-Csound".to_string()) }
    unsafe { CONFIG.as_mut().unwrap().upsampling_factor = config.upsample.unwrap_or(1); }
    unsafe { UPSAMPLING_FACTOR = config.upsample.unwrap_or(1); }
    unsafe { CONFIG.as_mut().unwrap().num_in_channels = config.audio_in.unwrap_or(0); }
    unsafe { CONFIG.as_mut().unwrap().num_out_channels = config.audio_out.unwrap_or(1); }
    unsafe { CONFIG.as_mut().unwrap().num_midi_in_ports = config.midi_in.unwrap_or(1); }
    unsafe { CONFIG.as_mut().unwrap().num_midi_out_ports = config.midi_out.unwrap_or(0); }

    for midi_connect in config.midi_connect.unwrap_or(Vec::new()) {
        let m_out = midi_connect.0;
        let m_in = midi_connect.1;

        let midi_out = if m_out == "keyboard" {
            keyboard_out()
        } else {
            match m_out.split_once(":") {
                Some((m_out_id, m_out_port)) => {
                    let m_out_id = m_out_id.trim();
                    let m_out_port = match m_out_port.trim().parse::<u32>() {
                        Ok(v) => v,
                        Err(_) => panic!("\nError parsing midi_out port: {}\n", m_out_port)
                    };
    
                    if m_out_id == "self" {
                        self_midi_out(m_out_port)
                    } else if m_out_id == "reaper" {
                        reaper_midi_out(m_out_port)
                    } else {
                        panic!("\nUnknown midi_out id: {}\n", m_out_id)
                    }
                },
                None => panic!("\nNo colon in midi_connect\n")
            }
        };
    
        let midi_in = match m_in.split_once(":") {
            Some((m_in_id, m_in_port)) => {
                let m_in_id = m_in_id.trim();
                let m_in_port = match m_in_port.trim().parse::<u32>() {
                    Ok(v) => v,
                    Err(_) => panic!("\nError parsing midi_in port: {}\n", m_in_port)
                };
    
                if m_in_id == "self" {
                    self_midi_in(m_in_port)
                } else if m_in_id == "reaper" {
                    reaper_midi_in(m_in_port)
                } else {
                    panic!("\nUnknown midi_in id: {}\n", m_in_id)
                }
            },
            None => panic!("\nNo colon in midi_connect\n")
        };

        connect(midi_out, midi_in);
    }

    for audio_connect in config.audio_connect.unwrap_or(Vec::new()) {
        let a_out = audio_connect.0;
        let a_in = audio_connect.1;

        let audio_out = match a_out.split_once(":") {
            Some((a_out_id, a_out_port)) => {
                let a_out_id = a_out_id.trim();
                let a_out_port = match a_out_port.trim().parse::<u32>() {
                    Ok(v) => v,
                    Err(_) => panic!("\nError parsing audio_out port: {}\n", a_out_port)
                };

                if a_out_id == "self" {
                    self_out(a_out_port)
                } else if a_out_id == "system" {
                    system_capture(a_out_port)
                } else if a_out_id == "reaper" {
                    reaper_out(a_out_port)
                } else {
                    panic!("\nUnknown audio_out id: {}\n", a_out_id)
                }
            },
            None => panic!("\nNo colon in audio_connect\n")
        };
        
        let audio_in = match a_in.split_once(":") {
            Some((a_in_id, a_in_port)) => {
                let a_in_id = a_in_id.trim();
                let a_in_port = match a_in_port.trim().parse::<u32>() {
                    Ok(v) => v,
                    Err(_) => panic!("\nError parsing audio_in port: {}\n", a_in_port)
                };

                if a_in_id == "self" {
                    self_in(a_in_port)
                } else if a_in_id == "system" {
                    system_playback(a_in_port)
                } else if a_in_id == "reaper" {
                    reaper_in(a_in_port)
                } else {
                    panic!("\nUnknown audio_in id: {}\n", a_in_id)
                }
            },
            None => panic!("\nNo colon in audio_connect\n")
        };
        
        connect(audio_out, audio_in);
    }

    for param_args in config.param.unwrap_or(Vec::new()) {
        let param_type_str = &param_args[0];
        let name = &param_args[1];
        let default_value = match param_args[2].parse::<f64>() {
            Ok(v) => v,
            Err(_) => panic!("\nError parsing param value: {}\n", param_args[2])
        };
        let param_type = match param_type_str.as_str() {
            "lin" => {
                let min = match param_args[3].parse::<f64>() {
                    Ok(v) => v,
                    Err(_) => panic!("\nError parsing lin param min value: {}\n", param_args[3])
                };
                let max = match param_args[4].parse::<f64>() {
                    Ok(v) => v,
                    Err(_) => panic!("\nError parsing lin param max value: {}\n", param_args[4])
                };
                let digits = match param_args[5].parse::<i32>() {
                    Ok(v) => v,
                    Err(_) => panic!("\nError parsing lin param digits value: {}\n", param_args[5])
                };
                ParamType::Linear(min, max, digits)
            },
            "exp" => {
                let min = match param_args[3].parse::<f64>() {
                    Ok(v) => v,
                    Err(_) => panic!("\nError parsing exp param min value: {}\n", param_args[3])
                };
                let max = match param_args[4].parse::<f64>() {
                    Ok(v) => v,
                    Err(_) => panic!("\nError parsing exp param max value: {}\n", param_args[4])
                };
                let digits = match param_args[5].parse::<i32>() {
                    Ok(v) => v,
                    Err(_) => panic!("\nError parsing exp param digits value: {}\n", param_args[5])
                };
                ParamType::Exponential(min, max, digits)
            },
            _ => panic!("\nUnknown param type: {}\n", param_type_str)
        };

        params.push(Param::new(&name, default_value, param_type))
    }
    
    for mr in config.midi_routing.unwrap_or(Vec::new()) {
        match mr.mode {
            midi::Mode::Mono => {
                unsafe { midi::MIDI_ROUTINGS.push(midi::MidiRoutingAndModeData { mr, md: midi::ModeData::Mono(None) }); }
            },
            midi::Mode::Poly => {
                unsafe { midi::MIDI_ROUTINGS.push(midi::MidiRoutingAndModeData { mr, md: midi::ModeData::Poly }); }
            },
            midi::Mode::PolyTrig => {
                unsafe { midi::MIDI_ROUTINGS.push(midi::MidiRoutingAndModeData { mr, md: midi::ModeData::PolyTrig(0) }); }
            }
        }
    }

    let csd = &csd[config_end+9..];
    csd.to_string()
}

pub fn init2(csd: String, config: &Config) {
    let cs_instruments_end = csd.find("<CsInstruments>").unwrap() + 15;
    let csd = format!("{}\nsr={}\nnchnls={}\nnchnls_i={}\n{}", &csd[..cs_instruments_end], 48000 * config.upsampling_factor, config.num_out_channels, config.num_in_channels, &csd[cs_instruments_end..]);
    let re_ksmps = Regex::new(r"ksmps\s*=\s*(\d+)").unwrap();
    let ksmps = re_ksmps.captures(&csd).expect("\nksmps not found in csd\n").get(1).unwrap().as_str().parse::<u64>().unwrap();

    unsafe {
        CSOUND = Some(csound::Csound::new());
        CSOUND.as_ref().unwrap().create_message_buffer(1);

        CSOUND.as_ref().unwrap().set_host_implemented_audioIO(1, BUFFER_SIZE * UPSAMPLING_FACTOR);

        // these options prevent csound from writing a wav file
        let _ = CSOUND.as_ref().unwrap().set_option("-odac");
        let _ = CSOUND.as_ref().unwrap().set_option("-+rtaudio=null");

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
        IN_CH = config.num_in_channels;
        OUT_CH = config.num_out_channels;
    }

}

pub fn init(csd: &str, ksmps: u64, config: &Config) {
    let csd = std::fs::read_to_string(csd).unwrap();
    let cs_instruments_end = csd.find("<CsInstruments>").unwrap() + 15;
    let csd = format!("{}\nsr={}\nksmps={}\nnchnls={}\n{}", &csd[..cs_instruments_end], 48000 * config.upsampling_factor, ksmps, config.num_out_channels, &csd[cs_instruments_end..]);

    unsafe {
        CSOUND = Some(csound::Csound::new());
        CSOUND.as_ref().unwrap().create_message_buffer(1);

        // these options prevent csound from writing a wav file
        let _ = CSOUND.as_ref().unwrap().set_option("-odac");
        let _ = CSOUND.as_ref().unwrap().set_option("-+rtaudio=null");

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

pub fn process_next_block(samples: &Vec<Vec<f64>>, params: &Vec<Param>) {
    for i in 0..params.len() {
        unsafe { CSOUND.as_mut().unwrap().set_control_channel(&format!("{}", params[i].name), params[i].value); }
    }

    unsafe {
        let mut spin = CSOUND.as_ref().unwrap().get_input_buffer().unwrap();
            
        for ch in 0..IN_CH as usize {
            for i in 0..(BUFFER_SIZE * UPSAMPLING_FACTOR) as usize {
                spin[i * IN_CH as usize + ch] = samples[ch][i];
            }
        }
    }
}

pub fn process(time_in_samples: u64, _params: &Vec<Param>, samples: &mut [f64; 32], done: &mut bool) {
    if *done {
        return;
    }

    unsafe {
        
        if time_in_samples % KSMPS as u64 == 0 {
            *done = CSOUND.as_ref().unwrap().perform_ksmps();
        }

        let spout = CSOUND.as_ref().unwrap().get_output_buffer().unwrap();

        let pos_in_buffer_window = time_in_samples % (BUFFER_SIZE * UPSAMPLING_FACTOR) as u64;
        
        for ch in 0..OUT_CH as usize {
            samples[ch] = spout[pos_in_buffer_window as usize * OUT_CH as usize + ch];
        }
    }
}

pub fn send_instr_event(pfields: &[f64]) {
    unsafe {
        CSOUND.as_ref().unwrap().send_score_event('i', pfields);
    }
}