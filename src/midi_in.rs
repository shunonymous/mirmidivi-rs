// SPDX-License-Identifier: GPL-3.0-or-later
use std::time::Instant;
use time::Duration;

use crossbeam_channel::{unbounded, Receiver};
use midir::{MidiInput, MidiInputConnection};

use crate::{options::Options, MidiData};

/// Midi Input
pub struct MidiIn {
    /// Midi Input connection
    midi_in_connection: Option<MidiInputConnection<()>>,
    epoch: Option<Instant>,
}

impl MidiIn {
    pub fn get_epoch(&self) -> Instant {
        self.epoch.expect("Something happened in connect()")
    }

    /// Connecting to MIDI sequencer (like alsa_seq, jack, etc...)
    pub fn connect(&mut self) -> Receiver<MidiData> {
        let midi_in = MidiInput::new("mirmidivi-rs").unwrap();
        let in_ports = midi_in.ports();
        let in_port = &in_ports[0];

        let in_port_name = midi_in.port_name(in_port).unwrap();

        let (s, r) = unbounded();

        self.epoch = Some(Instant::now());
        self.midi_in_connection = Some(
            midi_in
                .connect(
                    in_port,
                    &in_port_name,
                    move |stamp, message, _| {
                        let _send = s.send(MidiData {
                            message: message.to_vec(),
                            timestamp: Duration::microseconds(stamp as i64),
                        });
                    },
                    (),
                )
                .unwrap(),
        );

        r
    }

    pub fn new(opts: &Options) -> Self {
        MidiIn {
            midi_in_connection: None,
            epoch: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MidiIn;
    use crate::Options;

    #[test]
    fn add_on_event_instance() {
        let opts: Options = Options {
            renderer: "text".to_owned(),
        };
        let midi_in = MidiIn::new(&opts);
    }
}
