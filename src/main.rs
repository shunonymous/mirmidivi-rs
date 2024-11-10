// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use clap::Parser;
use ctrlc;
use midi_in::MidiIn;
use options::Options;
use time::Duration;
mod midi_in;
mod options;
mod renderer;
mod renderer_lib;

type Message = Vec<u8>;

#[derive(Debug)]
struct MidiData {
    message: Message,
    timestamp: Duration,
}

fn main() {
    let opts: Options = Options::parse();
    let quit = Arc::new(AtomicBool::new(false));
    let mut handlers = Vec::<JoinHandle<()>>::new();
    let q = quit.clone();
    let _ = ctrlc::set_handler(move || {
        q.store(true, SeqCst);
    });
    let mut midi_in: MidiIn = MidiIn::new(&opts);
    let midi_recv = midi_in.connect();
    let midi_in_epoch = midi_in.get_epoch();
    renderer::render_init(
        &opts,
        &midi_recv,
        midi_in_epoch,
        quit.clone(),
        &mut handlers,
    );

    loop {
        if quit.load(SeqCst) {
            handlers.into_iter().for_each(|t| {
                t.join().unwrap();
            });
            break;
        }
    }
}
