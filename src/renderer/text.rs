// SPDX-License-Identifier: GPL-3.0-or-later
use std::{
    io::Write,
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::{midi::MidiProvider, options::Options, renderer::Renderer, MidiData};
use crossbeam_channel::{select, Receiver, RecvError};
use midi_msg::{self, MidiMsg, ReceiverContext};
use std::sync::atomic::Ordering::SeqCst;
use std::time::Instant;

/// Rendering as a text
pub struct TextRenderer {}

impl TextRenderer {
    /// Output message to console.
    fn draw(midi: &MidiData) {
        let mut ctx = ReceiverContext::new();
        let message = format!(
            "{:?}",
            MidiMsg::from_midi_with_context(midi.message.as_slice(), &mut ctx)
                .expect("Not an error")
        );
        print!("\r{}[K", 27 as char); // Carriege return, Erase to end of line.
        print!("{}", message.trim_end());
        std::io::stdout().flush().unwrap();
    }
}

impl<T: MidiProvider> Renderer<T> for TextRenderer {
    fn init(
        _opts: &Options,
        midi: &T,
        quit: Arc<AtomicBool>,
        handlers: &mut Vec<JoinHandle<()>>,
    ) -> TextRenderer {
        let midi_recv = midi.get_midi_in_recv();
        handlers.push(thread::spawn(move || loop {
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
            }
            if quit.load(SeqCst) {
                break;
            }
        }));

        TextRenderer {}
    }
}
