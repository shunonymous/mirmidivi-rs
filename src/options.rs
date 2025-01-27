// SPDX-License-Identifier: GPL-3.0-or-later
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    /// API for rendering midi
    #[clap(short, long, value_parser, default_value_t = String::from("text"))]
    pub renderer: String,
    /// MIDI file for rendering
    #[clap(short, long, value_parser)]
    pub midifile: Option<String>,
}
