// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Instant;

use crossbeam_channel::Receiver;
use time::Duration;

use crate::{options::Options, MidiData};
pub use midi_in::MidiIn;
pub use midi_player::MidiPlayer;

mod midi_in;
mod midi_player;

pub trait MidiProvider {
    fn get_midi_in_recv(&self) -> Receiver<MidiData>;
    fn get_epoch(&self) -> Instant;
    // fn get_ellapsed(&self) -> Duration;
    fn new(opts: &Options) -> Self;
}
