use crate::{config::Config, state::InputMode};
use std::io;
use std::sync::mpsc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::{thread, time::Duration};

use termion::event::Key;
use termion::input::TermRead;

pub enum Event {
    Input(Key),
    InputPop,
    InputPush(char),
    ChangeInputMode(InputMode),
    SubmitInput,
    Tick,
    YoutubeQuery(String),
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event>,
    input_handle: thread::JoinHandle<()>,
    ignore_exit_key: Arc<AtomicBool>,
    tick_handle: thread::JoinHandle<()>,
}

impl Events {
    pub fn new(exit_key: Key, tick_rate: Duration) -> Events {
        let (tx, rx) = mpsc::channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));
        let input_handle = {
            let tx = tx.clone();
            let ignore_exit_key = ignore_exit_key.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(Event::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                        if !ignore_exit_key.load(Ordering::Relaxed) && key == exit_key {
                            return;
                        }
                    }
                }
            })
        };
        let tick_handle = {
            thread::spawn(move || loop {
                tx.send(Event::Tick).unwrap();
                thread::sleep(tick_rate);
            })
        };
        Events {
            rx,
            ignore_exit_key,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}
