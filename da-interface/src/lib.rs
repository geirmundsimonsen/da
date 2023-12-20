#[derive(Debug, Clone)]
pub struct Config {
    pub upsampling_factor: u32,
    pub num_in_channels: u32,
    pub num_out_channels: u32,
    pub num_midi_in_ports: u32,
    pub num_midi_out_ports: u32,
    pub name: String,
    pub connections: Vec<(String, String)>,
}

impl Config {
    pub fn new(
        name: &str,
        upsampling_factor: u32,
        num_in_channels: u32,
        num_out_channels: u32,
        num_midi_in_ports: u32,
        num_midi_out_ports: u32,
    ) -> Self {
        Self {
            upsampling_factor,
            num_in_channels,
            num_out_channels,
            num_midi_in_ports,
            num_midi_out_ports,
            name: name.to_string(),
            connections: vec![],
        }
    }
}

static mut CONFIG: Option<Config> = None;

pub fn make_config(
    name: &str,
    upsampling_factor: u32,
    num_in_channels: u32,
    num_out_channels: u32,
    num_midi_in_ports: u32,
    num_midi_out_ports: u32,
) {
    let cfg = Config::new(
        name,
        upsampling_factor,
        num_in_channels,
        num_out_channels,
        num_midi_in_ports,
        num_midi_out_ports,
    );
    unsafe {
        CONFIG = Some(cfg);
    }
}

pub fn config() -> &'static Config {
    unsafe { CONFIG.as_ref().unwrap() }
}

pub fn connect(from: String, to: String) {
    unsafe { CONFIG.as_mut().unwrap().connections.push((from, to)); }
}

pub fn self_in(channel: u32) -> String {
    format!("{}:in{}", unsafe { CONFIG.as_ref().unwrap().name.to_string() }, channel)
}

pub fn self_out(channel: u32) -> String {
    format!("{}:out{}", unsafe { CONFIG.as_ref().unwrap().name.to_string() }, channel)
}

pub fn self_midi_in(port: u32) -> String {
    format!( "{}:midi_in{}", unsafe { CONFIG.as_ref().unwrap().name.to_string() }, port)
}

pub fn self_midi_out(port: u32) -> String {
    format!( "{}:midi_out{}", unsafe { CONFIG.as_ref().unwrap().name.to_string() }, port)
}

pub fn system_capture(channel: u32) -> String {
    format!("system:capture_{}", channel)
}

pub fn system_playback(channel: u32) -> String {
    format!("system:playback_{}", channel)
}

pub fn keyboard_in() -> String {
    format!("a2j:A-Series Keyboard [28] (capture): A-Series Keyboard Keyboard")
}

#[derive(Debug)]
pub struct NoteOn {
    pub port: u8,
    pub channel: u8,
    pub note: u8,
    pub velocity: f64,
}

#[derive(Debug)]
pub struct NoteOff {
    pub port: u8,
    pub channel: u8,
    pub note: u8,
    pub velocity: f64,
}

#[derive(Debug)]
pub struct CC {
    pub port: u8,
    pub channel: u8,
    pub cc: u8,
    pub value: f64,
}

#[derive(Debug)]
pub struct PB {
    pub port: u8,
    pub channel: u8,
    pub value: f64,
}

#[derive(Debug)]
pub enum Midi {
    On(NoteOn),
    Off(NoteOff),
    CC(CC),
    PB(PB),
}
