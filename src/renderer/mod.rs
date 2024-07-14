use crossbeam_channel::Receiver;
// SPDX-License-Identifier: GPL-3.0-or-later
use mopa::mopafy;

use crate::options::Options;
use crate::{midi_in::*, Message, TimeStamp};

use self::text::TextRenderer;

mod text;

pub trait Renderer: mopa::Any {
    fn new(
        opts: &Options,
        midi_recv: &Receiver<(TimeStamp, Message)>,
        ctrlc: &Receiver<()>,
    ) -> Self
    where
        Self: Sized;
}
mopafy!(Renderer);

pub fn render_init(
    opts: &Options,
    midi_in: &mut MidiIn,
    midi_recv: &Receiver<(TimeStamp, Message)>,
    ctrlc: &Receiver<()>,
) {
    if opts.renderer == "text" {
        TextRenderer::new(opts, midi_recv, ctrlc);
    }
}
