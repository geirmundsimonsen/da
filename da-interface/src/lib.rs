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

    pub fn connect_to(&mut self, out_channel: u32, to: String) {
        let from = format!("{}:out{}", self.name, out_channel);
        self.connections.push((from, to));
    }

    pub fn connect_from(&mut self, in_channel: u32, from: String) {
        let to = format!("{}:in{}", self.name, in_channel);
        self.connections.push((from, to));
    }

    pub fn connect_midi_to(&mut self, out_port: u32, to: String) {
        let from = format!("{}:midi_out{}", self.name, out_port);
        self.connections.push((from, to));
    }

    pub fn connect_midi_from(&mut self, in_port: u32, from: String) {
        let to = format!("{}:midi_in{}", self.name, in_port);
        self.connections.push((from, to));
    }
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
