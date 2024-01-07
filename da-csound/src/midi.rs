use da_interface::{Midi, MsgType};
use serde::Deserialize;

pub static mut MIDI_ROUTINGS: [Option<MidiRoutingAndModeData>; 16] = [None; 16];

#[derive(Copy, Clone, Deserialize, PartialEq, Debug)]
pub enum Mode {
    Midi,
    Mono,
    Poly,
    PolyTrig,
}

#[derive(Copy, Clone)]
pub enum ModeData {
    Midi,
    Mono(Option<u8>),
    Poly,
    PolyTrig(i32)
}

#[derive(Copy, Clone, Deserialize, PartialEq, Debug)]
pub struct MidiRouting {
    pub mode: Mode,
    pub channel: u8,
    pub instr: u16,
}

#[derive(Copy, Clone)]
pub struct MidiRoutingAndModeData {
    pub mr: MidiRouting,
    pub md: ModeData
}

pub fn process_midi(midi: &Midi) {

    if let Some(mut mramd) = unsafe { MIDI_ROUTINGS[midi.channel as usize] } {
        match mramd.mr.mode {
            Mode::Midi => {
                match &midi.msg_type {
                    MsgType::NoteOn(on) => crate::send_instr_event(&vec![mramd.mr.instr as f64, 0.0, 0.0, 144.0, on.note as f64, on.velocity as f64]),
                    MsgType::NoteOff(off) => crate::send_instr_event(&vec![mramd.mr.instr as f64, 0.0, 0.0, 128.0, off.note as f64, off.velocity as f64]),
                    MsgType::CC(cc) => unsafe { crate::CSOUND.as_mut().unwrap().set_control_channel(&format!("{}:{}", midi.channel, cc.cc), cc.value) },
                    _ => ()
                }
            },
            Mode::Mono => {
                match &midi.msg_type {
                    MsgType::NoteOn(on) => {
                        crate::send_instr_event(&vec![mramd.mr.instr as f64, 0.0, -1.0, on.note as f64, on.velocity as f64]);
                        mramd.md = ModeData::Mono(Some(on.note));
                    },
                    MsgType::NoteOff(off) => {
                        if let ModeData::Mono(Some(note)) = mramd.md {
                            if note == off.note {
                                crate::send_instr_event(&vec![mramd.mr.instr as f64 * -1.0, 0.0, 0.0, off.note as f64, off.velocity as f64]);
                                mramd.md = ModeData::Mono(None);
                            }
                        }
                    },
                    MsgType::CC(cc) => unsafe { crate::CSOUND.as_mut().unwrap().set_control_channel(&format!("{}:{}", midi.channel, cc.cc), cc.value) },
                    _ => ()
                }
            },
            Mode::Poly => {
                match &midi.msg_type {
                    MsgType::NoteOn(on) => {
                        crate::send_instr_event(&vec![mramd.mr.instr as f64 + on.note as f64 / 1000.0, 0.0, -1.0, on.note as f64, on.velocity as f64]);
                    },
                    MsgType::NoteOff(off) => {
                        crate::send_instr_event(&vec![(mramd.mr.instr as f64 + off.note as f64 / 1000.0) * -1.0, 0.0, -1.0, off.note as f64, off.velocity as f64]);
                    },
                    MsgType::CC(cc) => unsafe { crate::CSOUND.as_mut().unwrap().set_control_channel(&format!("{}:{}", midi.channel, cc.cc), cc.value) },
                    _ => ()
                }
            },
            Mode::PolyTrig => {
                match &midi.msg_type {
                    MsgType::NoteOn(on) => {
                        if let ModeData::PolyTrig(counter) = &mut mramd.md {
                            if *counter >= 10000 {
                                *counter = 0;
                            }
                            *counter += 1;
                            crate::send_instr_event(&vec![mramd.mr.instr as f64 + *counter as f64 / 10000.0, 0.0, -1.0, on.note as f64, on.velocity as f64]);
                        }
                    },
                    MsgType::CC(cc) => unsafe { crate::CSOUND.as_mut().unwrap().set_control_channel(&format!("{}:{}", midi.channel, cc.cc), cc.value) },
                    _ => ()
                }
            }
        }
    }
}