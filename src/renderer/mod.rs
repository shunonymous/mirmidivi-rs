// SPDX-License-Identifier: GPL-3.0-or-later

use crossbeam_channel::Receiver;
use curses::CursesRenderer;
use mopa::mopafy;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Instant;

use crate::midi::MidiProvider;
use crate::options::Options;
use crate::MidiData;

use self::text::TextRenderer;
mod curses;
mod text;

pub trait Renderer: mopa::Any {
    fn init(
        opts: &Options,
        midi: &MidiProvider,
        quit: Arc<AtomicBool>,
        handlers: &mut Vec<JoinHandle<()>>,
    ) -> Self
    where
        Self: Sized;
}
mopafy!(Renderer);

pub fn render_init(
    opts: &Options,
    midi: &MidiProvider,
    quit: Arc<AtomicBool>,
    handlers: &mut Vec<JoinHandle<()>>,
) {
    if opts.renderer == "text" {
        TextRenderer::init(opts, &midi, quit, handlers);
    } else if opts.renderer == "curses" {
        CursesRenderer::init(opts, &midi, quit, handlers);
    }
}
