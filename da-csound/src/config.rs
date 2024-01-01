use serde::Deserialize;

use crate::midi::MidiRouting;

#[derive(Deserialize, PartialEq, Debug)]
pub struct MidiConnect(pub String, pub String);

#[derive(Deserialize, PartialEq, Debug)]
pub struct AudioConnect(pub String, pub String);

#[derive(Deserialize, PartialEq, Debug)]
pub struct YmlConfig {
    pub name: String,
    pub upsample: u32,
    pub audio_in: u32,
    pub audio_out: u32,
    pub midi_in: u32,
    pub midi_out: u32,
    pub midi_connect: Vec<MidiConnect>,
    pub audio_connect: Vec<AudioConnect>,
    pub param: Vec<Vec<String>>,
    pub midi_routing: Vec<MidiRouting>,
}

pub fn parse_config(yml: &str) -> YmlConfig {
    serde_yaml::from_str(yml).unwrap()
}