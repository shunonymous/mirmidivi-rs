use std::sync::{atomic::AtomicBool, Arc};

use crossbeam_channel::Receiver;

use crate::MidiData;

pub mod pianoroll;

pub trait RenderLib {
    fn new(midi_recv: &Receiver<MidiData>, quit: Arc<AtomicBool>) -> Self;
}
