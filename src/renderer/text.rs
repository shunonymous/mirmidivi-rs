// SPDX-License-Identifier: GPL-3.0-or-later
use std::io::Write;

use crate::{options::Options, renderer::Renderer};
use midi_msg::{self, MidiMsg, ReceiverContext};

use super::OnMidiInEvent;

/// Rendering as a text
pub struct TextRenderer {
    /// Text to displaying to console
    message: String,
}

impl OnMidiInEvent for TextRenderer {
    /// Executing process on MIDI event TextRenderer.
    fn on_event(&mut self, stamp: u64, msg: &[u8]) {
        let mut ctx = ReceiverContext::new();
        self.message = format!(
            "{:?}",
            MidiMsg::from_midi_with_context(msg, &mut ctx).expect("Not an error")
        );
        self.draw()
    }
}

impl Renderer for TextRenderer {
    /// Output message to console.
    fn draw(&self) {
        print!("\r{}[K", 27 as char); // Carriege return, Erase to end of line.
        print!("{}", self.message.trim_end());
        std::io::stdout().flush().unwrap();
    }

    fn new(_opts: &Options) -> TextRenderer {
        TextRenderer {
            message: String::new(),
        }
    }
}
