// SPDX-License-Identifier: GPL-3.0-or-later
use mopa::mopafy;

use crate::midi_in::*;
use crate::options::Options;

use self::text::TextRenderer;

mod text;

pub trait Renderer: mopa::Any {
    fn draw(&self);
    fn new(opts: &Options) -> Self
    where
        Self: Sized;
}
mopafy!(Renderer);

pub fn render_init(opts: &Options, midi_in: &mut MidiIn) {
    if opts.renderer == "text" {
        midi_in.add_on_event_instance(Box::new(TextRenderer::new(&opts)));
    }
}
