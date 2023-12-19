use std::{sync::Mutex, ffi::c_void, f32::NAN, collections::HashMap, borrow::BorrowMut};
use jack::{Client, ClientOptions, MidiIn, AudioOut, Port, AudioIn, AsyncClient, ProcessScope, Control, ProcessHandler, RawMidi, MidiOut};
use libloading::{Library, Symbol};
use da_interface::{Config, Midi, NoteOn, NoteOff, CC, PB};

use crate::{constants::MAX_JACK_FRAMES, param};

pub static CLIENT: Mutex<Option<Client>> = Mutex::new(None);
pub static IN_PORTS: Mutex<Vec<Port<AudioIn>>> = Mutex::new(Vec::new());
pub static OUT_PORTS: Mutex<Vec<Port<AudioOut>>> = Mutex::new(Vec::new());
pub static MIDI_IN_PORTS: Mutex<Vec<Port<MidiIn>>> = Mutex::new(Vec::new());
pub static MIDI_OUT_PORTS: Mutex<Vec<Port<MidiOut>>> = Mutex::new(Vec::new());

pub struct ConcreteProcessHandler {
    pub samples: Vec<Vec<f64>>,
    pub lib: libloading::Library,
    pub upsampling_factor: u32,
    pub num_in_channels: u32,
    pub num_out_channels: u32,
    pub num_midi_in_ports: u32,
    pub num_midi_out_ports: u32,
    pub channel_sample_buf: [f64; 32],
}

impl ConcreteProcessHandler {
    pub fn new(lib: libloading::Library, upsampling_factor: u32, num_in_channels: u32, num_out_channels: u32, num_midi_in_ports: u32, num_midi_out_ports: u32) -> Self {
        let samples = vec![vec![0.0f64; (MAX_JACK_FRAMES * upsampling_factor as u32) as usize]; num_out_channels as usize];
        ConcreteProcessHandler {
            samples,
            lib,
            upsampling_factor,
            num_in_channels,
            num_out_channels,
            num_midi_in_ports,
            num_midi_out_ports,
            channel_sample_buf: [0.0f64; 32],
        }
    }
}

static mut TIME_IN_SAMPLES: u64 = 0;

impl ProcessHandler for ConcreteProcessHandler {
    const SLOW_SYNC:bool = false;

    fn process(&mut self, _: &Client, ps: &ProcessScope) -> Control {
        IN_PORTS.lock().unwrap().iter_mut().enumerate().for_each(|(ch, port)| {
            if ch > self.num_out_channels as usize {
                return;
            }
            let in_port_buf = port.as_slice(ps);
            for i in 0..ps.n_frames() {
                for j in 0..self.upsampling_factor {
                    self.samples[ch][(i * self.upsampling_factor + j) as usize] = in_port_buf[i as usize] as f64;
                }
            }
        });

        let midi_in_ports = &MIDI_IN_PORTS.lock().unwrap();

        let mut block_midi_events: HashMap<u32, Vec<Midi>> = HashMap::new();

        let mut port_no = 0;
        for port in midi_in_ports.iter() {
            let raw_midi_events: Vec<RawMidi> = port.iter(ps).collect();
            for raw_midi in raw_midi_events {
                let midi_event = if raw_midi.bytes[0] >= 0x90 && raw_midi.bytes[0] <= 0x9F {
                    Some(Midi::On(NoteOn {
                        port: port_no,
                        channel: raw_midi.bytes[0] & 0x0F,
                        note: raw_midi.bytes[1],
                        velocity: raw_midi.bytes[2] as f64 / 127.0,
                    }))
                } else if raw_midi.bytes[0] >= 0x80 && raw_midi.bytes[0] <= 0x8F {
                    Some(Midi::Off(NoteOff {
                        port: port_no,
                        channel: raw_midi.bytes[0] & 0x0F,
                        note: raw_midi.bytes[1],
                        velocity: raw_midi.bytes[2] as f64 / 127.0,
                    }))
                } else if raw_midi.bytes[0] >= 0xB0 && raw_midi.bytes[0] <= 0xBF {
                    Some(Midi::CC(CC {
                        port: port_no,
                        channel: raw_midi.bytes[0] & 0x0F,
                        cc: raw_midi.bytes[1],
                        value: raw_midi.bytes[2] as f64 / 127.0,
                    }))
                } else if raw_midi.bytes[0] >= 0xE0 && raw_midi.bytes[0] <= 0xEF {
                    Some(Midi::PB(PB {
                        port: port_no,
                        channel: raw_midi.bytes[0] & 0x0F,
                        value: (raw_midi.bytes[2] as u16 * 128 + raw_midi.bytes[1] as u16) as f64 / 16383.0,
                    }))
                } else {
                    None
                };
                
                if let Some(midi) = midi_event {
                    let midi_time = raw_midi.time * self.upsampling_factor as u32;
                    if block_midi_events.contains_key(&midi_time) {
                        block_midi_events.get_mut(&midi_time).unwrap().push(midi);
                    } else {
                        block_midi_events.insert(midi_time, vec![midi]);
                    }
                }
            }
            port_no += 1;
        }

        let mut params = param::PARAMS.lock().unwrap();
        
        unsafe {
            let next: libloading::Symbol<unsafe extern fn(&mut [f64; 32], u64, &Vec<Midi>, &mut Vec<Midi>, &mut Vec<f64>)> = self.lib.get(b"next").unwrap();

            let empty_midi_events = Vec::new();
            let mut empty_midi_events_out = Vec::new();
            let mut raw_midi_events_out: Vec<Vec<(u32, Vec<u8>)>> = Vec::new();

            for _ in 0..self.num_midi_out_ports {
                raw_midi_events_out.push(Vec::new());
            }

            for i in 0..ps.n_frames()*self.upsampling_factor as u32 {
                for j in 0..self.num_in_channels {
                    self.channel_sample_buf[j as usize] = self.samples[j as usize][i as usize];
                }

                let midi_events_ref = block_midi_events.get(&i);
                
                if let Some(midi_events) = midi_events_ref {
                    next(&mut self.channel_sample_buf, TIME_IN_SAMPLES, midi_events, &mut empty_midi_events_out, &mut params);
                } else {
                    next(&mut self.channel_sample_buf, TIME_IN_SAMPLES, &empty_midi_events, &mut empty_midi_events_out, &mut params);
                }

                if empty_midi_events_out.len() > 0 {
                    for midi in empty_midi_events_out.iter() {
                        match midi {
                            Midi::On(note_on) => {
                                raw_midi_events_out[note_on.port as usize].push((
                                    i / self.upsampling_factor as u32,
                                    vec![0x90 | note_on.channel, note_on.note, (note_on.velocity * 127.0) as u8],
                                ));
                            },
                            Midi::Off(note_off) => {
                                raw_midi_events_out[note_off.port as usize].push((
                                    i / self.upsampling_factor as u32,
                                    vec![0x80 | note_off.channel, note_off.note, (note_off.velocity * 127.0) as u8],
                                ));
                            },
                            Midi::CC(cc) => {
                                raw_midi_events_out[cc.port as usize].push((
                                    i / self.upsampling_factor as u32,
                                    vec![0xB0 | cc.channel, cc.cc, (cc.value * 127.0) as u8],
                                ));
                            },
                            
                            Midi::PB(pb) => {
                                raw_midi_events_out[pb.port as usize].push((
                                    i / self.upsampling_factor as u32,
                                    vec![0xE0 | pb.channel, (pb.value * 16383.0 / 128.0) as u8, (pb.value * 16383.0 % 128.0) as u8],
                                ));
                            },
                        }
                    }
                    empty_midi_events_out.clear();
                }

                for j in 0..self.num_out_channels {
                    self.samples[j as usize][i as usize] = self.channel_sample_buf[j as usize];
                }

                MIDI_OUT_PORTS.lock().unwrap().iter_mut().enumerate().for_each(|(ch, port)| {
                    let mut writer = port.writer(ps);
                    let raw_midi_events = &raw_midi_events_out[ch];
                    for raw_midi in raw_midi_events {
                        let raw_raw_midi = RawMidi {
                            time: raw_midi.0,
                            bytes: raw_midi.1.as_slice(),
                        };
                        writer.write(&raw_raw_midi).unwrap();
                    }
                });

                TIME_IN_SAMPLES += 1;
            }
        }
        
        OUT_PORTS.lock().unwrap().iter_mut().enumerate().for_each(|(ch, port)| {
            let out_port_buf = port.as_mut_slice(ps);
            for i in 0..ps.n_frames() {
                out_port_buf[i as usize] = self.samples[ch][(i * self.upsampling_factor) as usize] as f32;
            }
        });

        jack::Control::Continue
    }
}

pub fn create_jack_client(name: &str) {
    let (client, status) = Client::new(name, ClientOptions::NO_START_SERVER).unwrap();
    if !status.is_empty() {
        println!("JACK client status: {:?}", status);
    }
    *CLIENT.lock().unwrap() = Some(client);
}

pub fn create_in_port() {
    let mut in_ports = IN_PORTS.lock().unwrap();
    let client = CLIENT.lock().unwrap();
    let port = client.as_ref().unwrap().register_port(format!("in{}", in_ports.len() + 1).as_str(), AudioIn::default()).unwrap();
    in_ports.push(port);
}

pub fn create_out_port() {
    let mut out_ports = OUT_PORTS.lock().unwrap();
    let client = CLIENT.lock().unwrap();
    let port = client.as_ref().unwrap().register_port(format!("out{}", out_ports.len() + 1).as_str(), AudioOut::default()).unwrap();
    out_ports.push(port);
}

pub fn create_midi_in_port() {
    let mut midi_in_ports = MIDI_IN_PORTS.lock().unwrap();
    let client = CLIENT.lock().unwrap();
    let port = client.as_ref().unwrap().register_port(format!("midi_in{}", midi_in_ports.len() + 1).as_str(), MidiIn::default()).unwrap();
    midi_in_ports.push(port);
}

pub fn create_midi_out_port() {
    let mut midi_out_ports = MIDI_OUT_PORTS.lock().unwrap();
    let client = CLIENT.lock().unwrap();
    let port = client.as_ref().unwrap().register_port(format!("midi_out{}", midi_out_ports.len() + 1).as_str(), MidiOut::default()).unwrap();
    midi_out_ports.push(port);
}

pub fn activate_with_callback(concrete_process_handler: ConcreteProcessHandler) -> AsyncClient<(), ConcreteProcessHandler> {
    let mut client = CLIENT.lock().unwrap();  
    let async_client = client.take().unwrap().activate_async((), concrete_process_handler).unwrap();
    return async_client;
}

pub fn parse_connections(unparsed_connections: &str) -> Vec<(String, String)> {
    let mut connections = Vec::new();
    for connection in unparsed_connections.split(",") {
        let mut ports = connection.split("->");
        let a = ports.next().unwrap();
        let b = ports.next().unwrap();
        connections.push((a.to_string(), b.to_string()));
    }
    return connections;
}

pub fn connect_ports(client: &Client, out_port: &str, in_port: &str) {
    client.connect_ports_by_name(out_port, in_port).unwrap();
}

pub fn play(shared_lib: &str) {
    let lib = unsafe { Library::new(shared_lib).unwrap() };
    
    let config = unsafe { lib.get::<Symbol<unsafe extern fn() -> Config>>(b"init").unwrap()() };

    create_jack_client(&config.name);
    for _ in 0..config.num_in_channels {
        create_in_port();
    }

    for _ in 0..config.num_out_channels {
        create_out_port();
    }

    for _ in 0..config.num_midi_in_ports {
        create_midi_in_port();
    }

    for _ in 0..config.num_midi_out_ports {
        create_midi_out_port();
    }

    let concrete_process_handler = ConcreteProcessHandler::new(lib, config.upsampling_factor, config.num_in_channels, config.num_out_channels, config.num_midi_in_ports, config.num_midi_out_ports);
    let ac = activate_with_callback(concrete_process_handler);

    for (a, b) in config.connections {
        connect_ports(&ac.as_client(), &a, &b);
    }


    std::thread::sleep(std::time::Duration::from_secs(3600*24));
}