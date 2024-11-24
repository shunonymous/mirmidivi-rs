// SPDX-License-Identifier: GPL-3.0-or-later

use super::Renderer;
use crate::{
    options::Options,
    renderer_lib::{pianoroll::PianoRoll, RenderLib},
    MidiData,
};
use crossbeam_channel::{select, tick, Receiver};
use pancurses::*;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Instant,
};
use time::Duration;

pub struct Size {
    pub x: i32,
    pub y: i32,
}

const COLOR_LIGHTGREEN: i16 = 8;
const COLOR_DEEPPINK: i16 = 9;

pub struct CursesRenderer {}

impl CursesRenderer {
    fn init() -> Window {
        let window = initscr();

        window.keypad(true);
        nonl();
        cbreak();
        echo();

        // TODO: Configurable colors
        if has_colors() {
            start_color();
            init_color(COLOR_DEEPPINK, 1000, 78, 574);
            init_pair(1, COLOR_RED, COLOR_BLACK);
            init_pair(2, COLOR_GREEN, COLOR_BLACK);
            init_pair(3, COLOR_YELLOW, COLOR_BLACK);
            init_pair(4, COLOR_BLUE, COLOR_BLACK);
            init_pair(5, COLOR_CYAN, COLOR_BLACK);
            init_pair(6, COLOR_MAGENTA, COLOR_BLACK);
            init_pair(7, COLOR_WHITE, COLOR_BLACK);
            init_pair(8, COLOR_DEEPPINK, COLOR_BLACK);
            init_pair(9, COLOR_RED, COLOR_BLACK);
            init_pair(10, COLOR_GREEN, COLOR_BLACK);
            init_pair(11, COLOR_YELLOW, COLOR_BLACK);
            init_pair(12, COLOR_BLUE, COLOR_BLACK);
            init_pair(13, COLOR_CYAN, COLOR_BLACK);
            init_pair(14, COLOR_MAGENTA, COLOR_BLACK);
            init_pair(15, COLOR_WHITE, COLOR_BLACK);
            init_pair(16, COLOR_DEEPPINK, COLOR_BLACK);
        }

        window.erase();

        window.refresh();
        curs_set(0);
        window.clear();

        window
    }

    fn draw_buffer(window: &Window, pianoroll: &PianoRoll, midi_in_epoch: &Instant) {
        let s = window.get_max_yx();
        let term_size = Size { x: s.1, y: s.0 };

        let usecs_per_line = 10 * 1000; // 10ms
        let end = Duration::try_from(midi_in_epoch.elapsed()).unwrap();
        let begin = end - Duration::microseconds((term_size.x * usecs_per_line) as i64);

        let buf = pianoroll.draw(begin, end, term_size.x as u32);

        let mut x: i32 = 1;
        window.erase();
        buf.iter().for_each(|line| {
            line.iter().for_each(|atom| {
                window.attrset(COLOR_PAIR(atom.channel as chtype + 1));
                window.mvaddstr(term_size.y - atom.scale as i32, x, "|".to_owned());
            });
            x += 1;
        });
        window.refresh();
    }
}

impl Renderer for CursesRenderer {
    fn init(
        _opts: &Options,
        midi_recv: &Receiver<MidiData>,
        midi_in_epoch: Instant,
        quit: Arc<AtomicBool>,
        handlers: &mut Vec<JoinHandle<()>>,
    ) -> Self {
        let midi_recv = midi_recv.clone();
        handlers.push(thread::spawn(move || {
            let window = Self::init();
            let render_lib = PianoRoll::new(&midi_recv, quit.clone());
            // 20 fps
            let tick = tick(Duration::milliseconds(50).unsigned_abs());

            loop {
                select! {
                    recv(tick) -> _ => {
                        Self::draw_buffer(&window, &render_lib, &midi_in_epoch);
                    },
                }
                if quit.load(SeqCst) {
                    endwin();
                    render_lib.handler.join().unwrap();
                    break;
                }
            }
        }));
        Self {}
    }
}
