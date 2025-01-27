// SPDX-License-Identifier: GPL-3.0-or-later

use std::thread;
use std::time::Instant;
use std::{fs, sync::mpsc};

use crossbeam_channel::{unbounded, Receiver, Sender};
use nodi::midly::Timing;
use nodi::timers::TimeFormatError;
use nodi::{
    self,
    midly::{Format, Smf},
    timers::{ControlTicker, Ticker},
    Connection, MidiEvent, Player, Sheet,
};
use time::Duration;

use crate::{midi::MidiProvider, options::Options, MidiData};

pub struct MidiPlayer {
    epoch: Instant,
    pause_send: mpsc::Sender<()>,
    midi_recv: Receiver<MidiData>,
}

struct MidiPlayerConnection {
    epoch: Instant,
    midi_send: Sender<MidiData>,
}

impl MidiProvider for MidiPlayer {
    fn get_midi_in_recv(&self) -> Receiver<MidiData> {
        self.midi_recv.clone()
    }

    fn get_epoch(&self) -> Instant {
        self.epoch
    }

    fn new(opts: &Options) -> Self {
        let (midi_send, midi_recv) = unbounded();
        let epoch = Instant::now();
        let file = fs::read(&opts.midifile.clone().unwrap()).expect("Failed to open MIDI file");
        let Smf { header, tracks } = Smf::parse(&file).expect("Failed to parse MIDI file");
        let sheet = match header.format {
            Format::SingleTrack | Format::Sequential => Sheet::sequential(&tracks),
            Format::Parallel => Sheet::parallel(&tracks),
        };
        let (pause_send, pause_recv) = mpsc::channel();
        let t = match header.timing {
            Timing::Metrical(n) => Ok(n),
            _ => Err(TimeFormatError),
        };
        let timer = ControlTicker::new(t.unwrap().into(), pause_recv);
        let mut player = Player::new(timer, MidiPlayerConnection::new(midi_send, epoch));
        thread::spawn(move || player.play(&sheet));

        MidiPlayer {
            epoch,
            pause_send,
            midi_recv,
        }
    }
}

impl MidiPlayer {
    pub fn toggle_pause_resume(&mut self) {
        let _ = self.pause_send.send(());
    }
}

impl MidiPlayerConnection {
    pub fn new(send: Sender<MidiData>, epoch: Instant) -> Self {
        Self {
            midi_send: send,
            epoch,
        }
    }
}

impl Connection for MidiPlayerConnection {
    fn play(&mut self, event: MidiEvent) -> bool {
        let mut message = Vec::with_capacity(8);
        event.write(&mut message).unwrap();
        let _send = self.midi_send.send(MidiData {
            message,
            timestamp: Duration::try_from(self.epoch.elapsed()).unwrap(),
        });
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::midi::MidiProvider;
    use std::time::Instant;

    use crate::options::Options;

    use super::MidiPlayer;

    #[test]
    fn midi_player() {
        let opts: Options = Options {
            renderer: "text".to_owned(),
            midifile: Some("sample.mid".to_owned()),
        };
        let _midi_player = MidiPlayer::new(&opts);
    }

    #[test]
    #[should_panic]
    fn midi_player_not_exist_file() {
        let opts: Options = Options {
            renderer: "text".to_owned(),
            midifile: Some("/not/exist/file.mid".to_owned()),
        };
        let _midi_player = MidiPlayer::new(&opts);
    }
}
