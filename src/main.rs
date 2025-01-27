// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use clap::Parser;
use ctrlc;
use midi::MidiProvider;
use midi::{MidiIn, MidiPlayer};
use options::Options;
use renderer::curses::CursesRenderer;
use renderer::text::TextRenderer;
use renderer::Renderer;
use time::Duration;
mod midi;
mod options;
mod renderer;
mod renderer_lib;

type Message = Vec<u8>;

#[derive(Debug)]
struct MidiData {
    message: Message,
    timestamp: Duration,
}

fn render_init<T: MidiProvider>(
    opts: &Options,
    midi: &T,
    quit: Arc<AtomicBool>,
    handlers: &mut Vec<JoinHandle<()>>,
) {
    if opts.renderer == "text" {
        TextRenderer::init(opts, midi, quit, handlers);
    } else if opts.renderer == "curses" {
        CursesRenderer::init(opts, midi, quit, handlers);
    } else {
        panic!("{} is not implemented for renderer", opts.renderer);
    }
}

fn main() {
    let opts: Options = Options::parse();
    let quit = Arc::new(AtomicBool::new(false));
    let mut handlers = Vec::<JoinHandle<()>>::new();
    let q = quit.clone();
    let _ = ctrlc::set_handler(move || {
        q.store(true, SeqCst);
    });

    match &opts.midifile {
        Some(_) => {
            let midi = MidiPlayer::new(&opts);
            render_init(&opts, &midi, quit.clone(), &mut handlers);
        }
        None => {
            let midi = MidiIn::new(&opts);
            render_init(&opts, &midi, quit.clone(), &mut handlers);
        }
    };

    loop {
        if quit.load(SeqCst) {
            handlers.into_iter().for_each(|t| {
                t.join().unwrap();
            });
            break;
        }
    }
}
