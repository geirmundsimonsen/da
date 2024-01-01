use da_interface::Midi;
use serde::Deserialize;

pub static mut MIDI_ROUTINGS: Vec<MidiRoutingAndModeData> = Vec::new();

#[derive(Deserialize, PartialEq, Debug)]
pub enum Mode {
    Mono,
    Poly,
    PolyTrig,
}

pub enum ModeData {
    Mono(Option<u8>),
    Poly,
    PolyTrig
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct MidiRouting {
    pub mode: Mode,
    pub channel: u8,
    pub instr: u16,
}

pub struct MidiRoutingAndModeData {
    pub mr: MidiRouting,
    pub md: ModeData
}

pub fn process_midi(midi: &Midi) {
    for mramd in unsafe { MIDI_ROUTINGS.iter_mut() } {
        match mramd.mr.mode {
            Mode::Mono => {
                if let Midi::On(on) = midi {
                    if on.channel == mramd.mr.channel {
                        crate::send_instr_event(&vec![mramd.mr.instr as f64, 0.0, -1.0, on.note as f64, on.velocity as f64]);
                        mramd.md = ModeData::Mono(Some(on.note));
                    }
                } else if let Midi::Off(off) = midi {
                    if off.channel == mramd.mr.channel {
                        if let ModeData::Mono(Some(note)) = mramd.md {
                            if note == off.note {
                                crate::send_instr_event(&vec![mramd.mr.instr as f64, 0.0, 0.0, off.note as f64, off.velocity as f64]);
                                mramd.md = ModeData::Mono(None);
                            }
                        }
                    }
                }
            },
            Mode::Poly => {
                if let Midi::On(on) = midi {
                    if on.channel == mramd.mr.channel {
                        crate::send_instr_event(&vec![mramd.mr.instr as f64 + on.note as f64 / 1000.0, 0.0, -1.0, on.note as f64, on.velocity as f64]);
                    }
                } else if let Midi::Off(off) = midi {
                    if off.channel == mramd.mr.channel {
                        crate::send_instr_event(&vec![mramd.mr.instr as f64 + off.note as f64 / 1000.0, 0.0, 0.0, off.note as f64, off.velocity as f64]);
                    }
                }
            },
            Mode::PolyTrig => {
                if let Midi::On(on) = midi {
                    if on.channel == mramd.mr.channel {
                        crate::send_instr_event(&vec![mramd.mr.instr as f64 + on.note as f64 / 1000.0, 0.0, -1.0, on.note as f64, on.velocity as f64]);
                    }
                }
            }
        }
    }
}