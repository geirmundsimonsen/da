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
