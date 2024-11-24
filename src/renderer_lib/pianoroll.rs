// SPDX-License-Identifier: GPL-3.0-or-later
use std::{
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc, RwLock,
    },
    thread::{self, JoinHandle},
};

use time::Duration;

use crossbeam_channel::{select, tick, Receiver};

use crate::MidiData;
use midi_msg::{Channel, ChannelVoiceMsg, MidiMsg};

use super::RenderLib;

#[derive(Debug)]
struct Note {
    begin: Duration,
    end: Option<Duration>,
    channel: Channel,
    note: u8,
    velocity: u8,
}

pub struct PianoRoll {
    pianoroll: Arc<RwLock<Vec<Note>>>,
    pub handler: JoinHandle<()>,
}

#[derive(Debug)]
pub struct Atom {
    pub scale: u8,
    pub channel: Channel,
}

pub type Line = Vec<Atom>;

impl PianoRoll {
    pub fn draw(&self, range_begin: Duration, range_end: Duration, sample_num: u32) -> Vec<Line> {
        let mut buf = Vec::<Line>::new();
        let p = self.pianoroll.clone();
        let pr = p.read().unwrap();

        let target_notes = pr.iter().filter(|note| {
            note.begin < range_end
                || (match note.end {
                    Some(note_end) => range_begin < note_end,
                    None => true,
                })
        });

        let mut cur_timestamp = range_begin;
        let interval = (range_end - range_begin) / sample_num;
        for sample in 0..sample_num {
            let mut line: Line = vec![];
            target_notes
                .clone()
                .filter(|note| {
                    note.begin < cur_timestamp
                        && match note.end {
                            Some(note_end) => cur_timestamp < note_end,
                            None => true,
                        }
                })
                .for_each(|note| {
                    let atom = Atom {
                        scale: note.note,
                        channel: note.channel,
                    };
                    line.push(atom);
                });
            buf.push(line);
            cur_timestamp += interval;
        }

        buf
    }

    fn on_event(pianoroll: &mut Vec<Note>, midi: &MidiData) {
        let result = MidiMsg::from_midi(midi.message.as_slice());

        match result {
            Ok(m) => match m.0 {
                MidiMsg::ChannelVoice { channel, msg } => match msg {
                    ChannelVoiceMsg::NoteOn { note, velocity } => pianoroll.push(Note {
                        begin: midi.timestamp,
                        end: None,
                        channel,
                        note,
                        velocity,
                    }),
                    ChannelVoiceMsg::NoteOff { note, .. } => {
                        pianoroll
                            .iter_mut()
                            .rfind(|n| (n.channel == channel && n.note == note))
                            .map(|n| n.end = Some(midi.timestamp));
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }
}

impl RenderLib for PianoRoll {
    fn new(midi_recv: &Receiver<MidiData>, quit: Arc<AtomicBool>) -> Self {
        let pianoroll = Arc::new(RwLock::new(Vec::<Note>::new()));
        let midi_recv = midi_recv.clone();
        let tick = tick(Duration::seconds(2).unsigned_abs());

        let p = pianoroll.clone();
        let handler = thread::spawn(move || loop {
            select! {
                recv(midi_recv) -> midi => {
                    match midi {
                        Ok (midi) => {
                            Self::on_event(&mut p.write().unwrap(), &midi);
                        }
                        Err(_) => {

                        }
                    }
                },
                recv(tick) -> _ => (),
            }
            if quit.load(SeqCst) {
                break;
            }
        });

        Self { pianoroll, handler }
    }
}

#[cfg(test)]
mod tests {
    use super::PianoRoll;
    use crate::{renderer_lib::RenderLib, MidiData};
    use crossbeam_channel::{bounded, unbounded};
    use midi_msg::{MidiMsg, ReceiverContext};
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering::SeqCst},
            Arc,
        },
        thread::sleep,
    };
    use time::Duration;

    #[test]
    fn pianoroll_new() {
        let quit = Arc::new(AtomicBool::new(false));
        let (midi_snd, midi_recv) = unbounded();

        let pianoroll = PianoRoll::new(&midi_recv, quit.clone());

        let midi_data = [
            MidiData {
                message: vec![
                    0x90, // Message: NOTE_ON, Channel: 1
                    0x60, // Note
                    0x64, // Velocity
                ],
                timestamp: Duration::seconds(2),
            },
            MidiData {
                message: vec![
                    0x80, // Message: NOTE_OFF, Channel: 1
                    0x60, // Note
                    0x64, // Velocity
                ],
                timestamp: Duration::seconds(3),
            },
            MidiData {
                message: vec![
                    0x90, // Message: NOTE_ON, Channel: 1
                    0x7F, // Note
                    0x64, // Velocity
                ],
                timestamp: Duration::seconds(4),
            },
            MidiData {
                message: vec![
                    0x80, // Message: NOTE_OFF, Channel: 1
                    0x7F, // Note
                    0x64, // Velocity
                ],
                timestamp: Duration::seconds(5),
            },
            MidiData {
                message: vec![
                    0x90, // Message: NOTE_ON, Channel: 1
                    0x01, // Note
                    0x64, // Velocity
                ],
                timestamp: Duration::seconds(6),
            },
            MidiData {
                message: vec![
                    0x80, // Message: NOTE_OFF, Channel: 1
                    0x01, // Note
                    0x64, // Velocity
                ],
                timestamp: Duration::seconds(7),
            },
        ];

        midi_data.into_iter().for_each(|data| {
            let _ = midi_snd.send(data);
        });

        sleep(Duration::seconds(1).unsigned_abs());

        let line = 100;
        let frame = pianoroll.draw(Duration::seconds(0), Duration::seconds(15), line);
        assert_eq!(frame.len(), line as usize);

        let mut elem_exists: bool = false;
        frame.iter().for_each(|elem| {
            if !elem.is_empty() {
                elem_exists = true;
            }
        });

        println!("{:#?}", frame);

        assert!(elem_exists);

        quit.store(true, SeqCst);
    }
}
