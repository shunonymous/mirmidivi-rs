// SPDX-License-Identifier: GPL-3.0-or-later
use clap::Parser;
use midi_in::MidiIn;
use options::Options;
mod midi_in;
mod options;
mod renderer;

fn main() {
    let opts: Options = Options::parse();
    let mut midi_in: MidiIn = MidiIn::new(&opts);
    midi_in.connect();
    renderer::render_init(&opts, &mut midi_in);

    loop {}
}
