// SPDX-License-Identifier: GPL-3.0-or-later
use crossbeam_channel::{unbounded, Receiver};
use midir::{MidiInput, MidiInputConnection};

use crate::{options::Options, Message, TimeStamp};

/// Midi Input
pub struct MidiIn {
    /// Midi Input connection
    midi_in_connection: Option<MidiInputConnection<()>>,
}

impl MidiIn {
    /// Connecting to MIDI sequencer (like alsa_seq, jack, etc...)
    pub fn connect(&mut self) -> Receiver<(TimeStamp, Message)> {
        let midi_in = MidiInput::new("mirmidivi-rs").unwrap();
        let in_ports = midi_in.ports();
        let in_port = &in_ports[0];

        let in_port_name = midi_in.port_name(in_port).unwrap();

        let (s, r) = unbounded();

        self.midi_in_connection = Some(
            midi_in
                .connect(
                    in_port,
                    &in_port_name,
                    move |stamp, message, _| {
                        let _send = s.send((stamp, message.to_vec()));
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
