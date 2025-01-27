// SPDX-License-Identifier: GPL-3.0-or-later
use std::time::Instant;
use time::Duration;

use crossbeam_channel::{unbounded, Receiver};
use midir::{MidiInput, MidiInputConnection};

use crate::{midi::MidiProvider, options::Options, MidiData};

/// Midi Input
pub struct MidiIn {
    /// Midi Input connection
    midi_recv: Receiver<MidiData>,
    epoch: Instant,
}

impl MidiProvider for MidiIn {
    fn get_midi_in_recv(&self) -> Receiver<MidiData> {
        self.midi_recv.clone()
    }

    fn get_epoch(&self) -> Instant {
        self.epoch
    }

    fn new(opts: &Options) -> Self {
        let midi_in = MidiInput::new("mirmidivi-rs").unwrap();
        let in_ports = midi_in.ports();
        let in_port = &in_ports[0];

        let in_port_name = midi_in.port_name(in_port).unwrap();

        let (midi_send, midi_recv) = unbounded();

        let _ = midi_in.connect(
            in_port,
            &in_port_name,
            move |stamp, message, _| {
                let _send = midi_send.send(MidiData {
                    message: message.to_vec(),
                    timestamp: Duration::microseconds(stamp as i64),
                });
            },
            (),
        );

        MidiIn {
            midi_recv,
            epoch: Instant::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::midi::MidiProvider;

    use crate::midi::MidiIn;
    use crate::Options;

    #[test]
    fn add_on_event_instance() {
        let opts: Options = Options {
            renderer: "text".to_owned(),
            midifile: None,
        };
        let midi_in = MidiIn::new(&opts);
    }
}
