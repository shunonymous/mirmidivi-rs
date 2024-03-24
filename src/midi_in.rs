// SPDX-License-Identifier: GPL-3.0-or-later
use midir::{MidiInput, MidiInputConnection};
use mopa::mopafy;
use std::{
    collections::LinkedList,
    sync::{Arc, RwLock},
};

use crate::options::Options;

/// Some process executing at received midi message.
pub trait OnMidiInEvent: Send + Sync + mopa::Any {
    fn on_event(&mut self, stamp: u64, msg: &[u8]);
}
mopafy!(OnMidiInEvent);

/// Midi Input
pub struct MidiIn {
    /// Midi Input connection
    midi_in_connection: Option<MidiInputConnection<()>>,
    /// Instances have process on midi event
    on_event_instances: Arc<RwLock<LinkedList<Box<dyn OnMidiInEvent>>>>,
}

impl MidiIn {
    /// Connecting to MIDI sequencer (like alsa_seq, jack, etc...)
    pub fn connect(&mut self) {
        let mut midi_in = MidiInput::new("mirmidivi-rs").unwrap();
        let in_ports = midi_in.ports();
        let in_port = &in_ports[0];

        let in_port_name = midi_in.port_name(in_port).unwrap();
        let mut on_event_instances = self.on_event_instances.clone();

        self.midi_in_connection = Some(
            midi_in
                .connect(
                    in_port,
                    &in_port_name,
                    move |stamp, message, _| {
                        for i in on_event_instances.write().unwrap().iter_mut() {
                            i.on_event(stamp, message);
                        }
                    },
                    (),
                )
                .unwrap(),
        );
    }

    pub fn add_on_event_instance(&mut self, i: Box<dyn OnMidiInEvent>) {
        self.on_event_instances.write().unwrap().push_back(i);
    }

    pub fn new(opts: &Options) -> Self {
        MidiIn {
            midi_in_connection: None,
            on_event_instances: Arc::new(RwLock::new(LinkedList::<Box<dyn OnMidiInEvent>>::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::midi_in::OnMidiInEvent;
    use crate::MidiIn;
    use crate::Options;

    struct OnMidiInEventTest {}

    impl OnMidiInEventTest {
        fn new() -> Self {
            OnMidiInEventTest {}
        }
    }

    impl OnMidiInEvent for OnMidiInEventTest {
        fn on_event(&mut self, stamp: u64, msg: &[u8]) {}
    }

    #[test]
    fn add_on_event_instance() {
        let opts: Options = Options {
            renderer: "text".to_owned(),
        };
        let mut midi_in = MidiIn::new(&opts);
        let i = OnMidiInEventTest::new();
        midi_in.add_on_event_instance(Box::new(i));
        assert_eq!(midi_in.on_event_instances.read().unwrap().len(), 1);
    }
}
