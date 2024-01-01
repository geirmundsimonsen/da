use core::panic;

use da_interface::{CONFIG, Config, Param, ParamType, connect, keyboard_out, self_out, system_playback, self_midi_in, self_midi_out, reaper_midi_out, reaper_midi_in, system_capture, reaper_out, self_in, reaper_in};
use regex::Regex;

static mut CSOUND: Option<csound::Csound> = None;
static mut KSMPS: u64 = 16;
static mut PARAM_UPDATE_SAMPLES: u64 = 48000;

pub fn get_csd_from_env() -> String {
    let csd_path = match std::env::var("DA_CSOUND") {
        Ok(csd) => csd,
        Err(_) => panic!("\nDA_CSOUND environment variable not set.\n")
    };

    std::fs::read_to_string(csd_path).expect("\nError reading CSD file.\n")
}

pub fn parse_config(csd: String, params: &mut Vec<Param>) -> String {
    // find index of <Config>
    let config_start = csd.find("<Config>").expect("\n<Config> block missing.\n");
    let config_end = csd.find("</Config>").expect("\n</Config> terminator missing\n");

    let config_dsl = &csd[config_start+9..config_end];
    config_dsl.lines().for_each(|line| {
        let mut key_value = line.split("=");

        let key = match key_value.next() {
            Some(k) => k.trim(),
            None => panic!("\nNo key before an =. Line: {}\n", line)
        };
        
        let value = match key_value.next() {
            Some(v) => v.trim(),
            None => panic!("\nNo value after an =. Line: {}\n", line)
        };
        
        match key {
            "name" => { unsafe { CONFIG.as_mut().unwrap().name = value.to_string(); }},
            "upsample" => {
                let upsampling_factor = match value.parse::<u32>() {
                    Ok(v) => v,
                    Err(_) => panic!("\nError parsing upsample value: {}. Line: {}\n", value, line)
                };
                unsafe { CONFIG.as_mut().unwrap().upsampling_factor = upsampling_factor; }
            },
            "audio_channels" => {
                let (num_in_channels, num_out_channels) = match value.split_once(",") {
                    Some((num_in_channels, num_out_channels)) => {
                        (
                            match num_in_channels.trim().parse::<u32>() {
                                Ok(v) => v,
                                Err(_) => panic!("\nError parsing audio_channels value: {}. Line: {}\n", value, line)
                            },
                            match num_out_channels.trim().parse::<u32>() {
                                Ok(v) => v,
                                Err(_) => panic!("\nError parsing audio_channels value: {}. Line: {}\n", value, line)
                            }
                        )
                    },
                    None => panic!("\nNo comma in audio_channels value: {}. Line: {}\n", value, line)
                };
                unsafe {
                    CONFIG.as_mut().unwrap().num_in_channels = num_in_channels;
                    CONFIG.as_mut().unwrap().num_out_channels = num_out_channels;
                }
            },
            "midi_ports" => {
                let (num_in_ports, num_out_ports) = match value.split_once(",") {
                    Some((num_in_ports, num_out_ports)) => {
                        (
                            match num_in_ports.trim().parse::<u32>() {
                                Ok(v) => v,
                                Err(_) => panic!("\nError parsing midi_ports value: {}. Line: {}\n", value, line)
                            },
                            match num_out_ports.trim().parse::<u32>() {
                                Ok(v) => v,
                                Err(_) => panic!("\nError parsing midi_ports value: {}. Line: {}\n", value, line)
                            }
                        )
                    },
                    None => panic!("\nNo comma in midi_ports value: {}. Line: {}\n", value, line)
                };
                
                
                unsafe {
                    CONFIG.as_mut().unwrap().num_midi_in_ports = num_in_ports;
                    CONFIG.as_mut().unwrap().num_midi_out_ports = num_out_ports;
                }
            },
            "m_connect" => {
                let (m_out, m_in) = match value.split_once(",") {
                    Some((m_out, m_in)) => (m_out.trim(), m_in.trim()),
                    None => panic!("\nNo comma in m_connect value: {}. Line: {}\n", value, line)
                };

                let midi_out = if m_out == "keyboard" {
                    keyboard_out()
                } else {
                    match m_out.split_once(":") {
                        Some((m_out_id, m_out_port)) => {
                            let m_out_id = m_out_id.trim();
                            let m_out_port = match m_out_port.trim().parse::<u32>() {
                                Ok(v) => v,
                                Err(_) => panic!("\nError parsing midi_out port: {}. Line: {}\n", m_out_port, line)
                            };

                            if m_out_id == "self" {
                                self_midi_out(m_out_port)
                            } else if m_out_id == "reaper" {
                                reaper_midi_out(m_out_port)
                            } else {
                                panic!("\nUnknown midi_out id: {}. Line: {}\n", m_out_id, line)
                            }
                        },
                        None => panic!("\nNo colon in m_connect value: {}. Line: {}\n", value, line)
                    }
                };
                
                let midi_in = match m_in.split_once(":") {
                    Some((m_in_id, m_in_port)) => {
                        let m_in_id = m_in_id.trim();
                        let m_in_port = match m_in_port.trim().parse::<u32>() {
                            Ok(v) => v,
                            Err(_) => panic!("\nError parsing midi_in port: {}. Line: {}\n", m_in_port, line)
                        };

                        if m_in_id == "self" {
                            self_midi_in(m_in_port)
                        } else if m_in_id == "reaper" {
                            reaper_midi_in(m_in_port)
                        } else {
                            panic!("\nUnknown midi_in id: {}. Line: {}\n", m_in_id, line)
                        }
                    },
                    None => panic!("\nNo colon in m_connect value: {}. Line: {}\n", value, line)
                };
                
                connect(midi_out, midi_in);
            },
            "a_connect" => {
                let (a_out, a_in) = match value.split_once(",") {
                    Some((a_out, a_in)) => (a_out.trim(), a_in.trim()),
                    None => panic!("\nNo comma in a_connect value: {}. Line: {}\n", value, line)
                };

                let audio_out = match a_out.split_once(":") {
                    Some((a_out_id, a_out_port)) => {
                        let a_out_id = a_out_id.trim();
                        let a_out_port = match a_out_port.trim().parse::<u32>() {
                            Ok(v) => v,
                            Err(_) => panic!("\nError parsing audio_out port: {}. Line: {}\n", a_out_port, line)
                        };

                        if a_out_id == "self" {
                            self_out(a_out_port)
                        } else if a_out_id == "system" {
                            system_capture(a_out_port)
                        } else if a_out_id == "reaper" {
                            reaper_out(a_out_port)
                        } else {
                            panic!("\nUnknown audio_out id: {}. Line: {}\n", a_out_id, line)
                        }
                    },
                    None => panic!("\nNo colon in a_connect value: {}. Line: {}\n", value, line)
                };
                
                let audio_in = match a_in.split_once(":") {
                    Some((a_in_id, a_in_port)) => {
                        let a_in_id = a_in_id.trim();
                        let a_in_port = match a_in_port.trim().parse::<u32>() {
                            Ok(v) => v,
                            Err(_) => panic!("\nError parsing audio_in port: {}. Line: {}\n", a_in_port, line)
                        };

                        if a_in_id == "self" {
                            self_in(a_in_port)
                        } else if a_in_id == "system" {
                            system_playback(a_in_port)
                        } else if a_in_id == "reaper" {
                            reaper_in(a_in_port)
                        } else {
                            panic!("\nUnknown audio_in id: {}. Line: {}\n", a_in_id, line)
                        }
                    },
                    None => panic!("\nNo colon in a_connect value: {}. Line: {}\n", value, line)
                };
                
                connect(audio_out, audio_in);
            },
            "param" => {
                let param_args = value.splitn(4, ",").map(|s| s.trim()).collect::<Vec<&str>>();
                let param_type_str = param_args[0];
                let name = param_args[1];
                let default_value = match param_args[2].parse::<f64>() {
                    Ok(v) => v,
                    Err(_) => panic!("\nError parsing param value: {}. Line: {}\n", param_args[2], line)
                };
                let param_type = match param_type_str {
                    "lin" => {
                        let lin_param_args = param_args[3].splitn(3, ",").map(|s| s.trim()).collect::<Vec<&str>>();
                        let min = match lin_param_args[0].parse::<f64>() {
                            Ok(v) => v,
                            Err(_) => panic!("\nError parsing lin param min value: {}. Line: {}\n", lin_param_args[0], line)
                        };
                        let max = match lin_param_args[1].parse::<f64>() {
                            Ok(v) => v,
                            Err(_) => panic!("\nError parsing lin param max value: {}. Line: {}\n", lin_param_args[1], line)
                        };
                        let digits = match lin_param_args[2].parse::<i32>() {
                            Ok(v) => v,
                            Err(_) => panic!("\nError parsing lin param digits value: {}. Line: {}\n", lin_param_args[2], line)
                        };
                        ParamType::Linear(min, max, digits)
                    },
                    "exp" => {
                        let exp_param_args = param_args[3].splitn(3, ",").map(|s| s.trim()).collect::<Vec<&str>>();
                        let min = match exp_param_args[0].parse::<f64>() {
                            Ok(v) => v,
                            Err(_) => panic!("\nError parsing exp param min value: {}. Line: {}\n", exp_param_args[0], line)
                        };
                        let max = match exp_param_args[1].parse::<f64>() {
                            Ok(v) => v,
                            Err(_) => panic!("\nError parsing exp param max value: {}. Line: {}\n", exp_param_args[1], line)
                        };
                        let digits = match exp_param_args[2].parse::<i32>() {
                            Ok(v) => v,
                            Err(_) => panic!("\nError parsing exp param digits value: {}. Line: {}\n", exp_param_args[2], line)
                        };
                        ParamType::Exponential(min, max, digits)
                    },
                    _ => panic!("\nUnknown param type: {}. Line: {}\n", param_type_str, line)
                };


                params.push(Param::new(name, default_value, param_type))
            }
            _ => { 
                if !key.starts_with(";") && !key.starts_with("//") && !key.starts_with("#") {
                    panic!("Unknown key: {}. Line: {}", key, line);
                }
            }
        }
    });
    
    let csd = &csd[config_end+9..];
    csd.to_string()
}

pub fn init2(csd: String, config: &Config) {
    let cs_instruments_end = csd.find("<CsInstruments>").unwrap() + 15;
    let csd = format!("{}\nsr={}\nnchnls={}\n{}", &csd[..cs_instruments_end], 48000 * config.upsampling_factor, config.num_out_channels, &csd[cs_instruments_end..]);
    let re_ksmps = Regex::new(r"ksmps\s*=\s*(\d+)").unwrap();
    let ksmps = re_ksmps.captures(&csd).expect("\nksmps not found in csd\n").get(1).unwrap().as_str().parse::<u64>().unwrap();

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

        PARAM_UPDATE_SAMPLES = (48000 * config.upsampling_factor / 100) as u64;
    }

}

pub fn init(csd: &str, ksmps: u64, param_update_hz: u32, config: &Config) {
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
        PARAM_UPDATE_SAMPLES = (48000 * config.upsampling_factor / param_update_hz) as u64;
    }
}

pub fn process(time_in_samples: u64, params: &Vec<Param>, samples: &mut [f64; 32], done: &mut bool) {
    if *done {
        return;
    }

    unsafe {
        if time_in_samples % KSMPS as u64 == 0 {
            *done = CSOUND.as_ref().unwrap().perform_ksmps();
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