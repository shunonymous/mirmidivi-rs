// SPDX-License-Identifier: GPL-3.0-or-later
use std::{io::Write, thread};

use crate::{options::Options, renderer::Renderer, Message, TimeStamp};
use crossbeam_channel::{select, Receiver, RecvError};
use midi_msg::{self, MidiMsg, ReceiverContext};

/// Rendering as a text
pub struct TextRenderer {}

impl TextRenderer {
    /// Output message to console.
    fn draw(midi: &(TimeStamp, Message)) {
        let mut ctx = ReceiverContext::new();
        let message = format!(
            "{:?}",
            MidiMsg::from_midi_with_context(midi.1.as_slice(), &mut ctx).expect("Not an error")
        );
        print!("\r{}[K", 27 as char); // Carriege return, Erase to end of line.
        print!("{}", message.trim_end());
        std::io::stdout().flush().unwrap();
    }
}

impl Renderer for TextRenderer {
    fn new(
        _opts: &Options,
        midi_recv: &Receiver<(TimeStamp, Message)>,
        ctrlc: &Receiver<()>,
    ) -> TextRenderer {
        let midi_recv = midi_recv.clone();
        let ctrlc = ctrlc.clone();
        thread::spawn(move || loop {
            select! {
                recv(midi_recv) -> midi => {
                    match midi {
                        Ok(midi) => {
                            Self::draw(&midi);
                        },
                        Err(RecvError) => {
                            break;
                        },
                    }
                }
                recv(ctrlc) -> _ => {
                    break;
                }
            }
        });

        TextRenderer {}
    }
}
