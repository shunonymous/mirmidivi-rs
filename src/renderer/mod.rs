// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crate::midi::MidiProvider;
use crate::options::Options;

use self::text::TextRenderer;
pub mod curses;
pub mod text;

pub trait Renderer<T: MidiProvider> {
    fn init(
        opts: &Options,
        midi: &T,
        quit: Arc<AtomicBool>,
        handlers: &mut Vec<JoinHandle<()>>,
    ) -> Self;
}
