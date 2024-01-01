use serde::Deserialize;

use crate::midi::MidiRouting;

#[derive(Deserialize, PartialEq, Debug)]
pub struct MidiConnect(pub String, pub String);

#[derive(Deserialize, PartialEq, Debug)]
pub struct AudioConnect(pub String, pub String);

#[derive(Deserialize, PartialEq, Debug)]
pub struct YmlConfig {
    pub name: Option<String>,
    pub upsample: Option<u32>,
    pub audio_in: Option<u32>,
    pub audio_out: Option<u32>,
    pub midi_in: Option<u32>,
    pub midi_out: Option<u32>,
    pub midi_connect: Option<Vec<MidiConnect>>,
    pub audio_connect: Option<Vec<AudioConnect>>,
    pub param: Option<Vec<Vec<String>>>,
    pub midi_routing: Option<Vec<MidiRouting>>,
}

pub fn parse_config(yml: &str) -> YmlConfig {
    serde_yaml::from_str(yml).unwrap()
}