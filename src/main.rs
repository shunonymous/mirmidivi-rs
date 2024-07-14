// SPDX-License-Identifier: GPL-3.0-or-later
use clap::Parser;
use crossbeam_channel::select;
use crossbeam_channel::{bounded, Receiver};
use ctrlc;
use midi_in::MidiIn;
use options::Options;
mod midi_in;
mod options;
mod renderer;

type NoteScale = u8;
type TimeStamp = u64;
type Message = Vec<u8>;

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (snd, recv) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = snd.send(());
    })?;

    Ok(recv)
}

fn main() {
    let opts: Options = Options::parse();
    let ctrl_c_events = ctrl_channel().unwrap();
    let mut midi_in: MidiIn = MidiIn::new(&opts);
    let midi_recv = midi_in.connect();
    renderer::render_init(&opts, &mut midi_in, &midi_recv, &ctrl_c_events);

    loop {
        select! {
            recv(ctrl_c_events) -> _ => {
                println!("Quit.");
                break;
            }
        }
    }
}
