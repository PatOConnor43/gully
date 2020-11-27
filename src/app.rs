use crate::config;
use crate::events;
use crate::state;
use crate::ui;
use anyhow::Result;
use std::sync::mpsc::{Receiver, Sender};
use termion::event::Key;
use tui::{backend::Backend, Frame};
//use youtube_api::YoutubeApi;

pub enum AppLifecyle {
    Continue,
    Quit,
}

/// App holds the state of the application
pub struct App {
    background_event_tx: Sender<events::BackgroundActions>,
    config: config::Config,
    events: events::Events,
    state: state::State,
}

impl App {
    pub fn new(
        rx: Receiver<events::AppActions>,
        tx: Sender<events::BackgroundActions>,
        c: config::Config,
    ) -> Self {
        let mut app = App {
            background_event_tx: tx,
            config: c.clone(),
            events: events::Events::new(c.keys().exit_key(), c.behavior().tick_rate(), rx),
            state: state::State::default(),
        };
        app.background_event_tx
            .send(events::BackgroundActions::YoutubeQuery(
                "cat videos".to_owned(),
            ))
            .unwrap();
        app
    }
    pub fn dispatch(&mut self, e: events::Event) {
        match e {
            events::Event::ChangeInputMode(i) => {
                self.state.input_mode = i;
            }
            events::Event::InputPush(ch) => {
                self.state.input.push(ch);
            }
            events::Event::InputPop => {
                self.state.input.pop();
            }
            events::Event::SubmitInput => {
                self.state
                    .messages
                    .push(self.state.input.drain(..).collect());
            }
            _ => {}
        }
    }
    pub fn draw<B>(&self, f: &mut Frame<B>)
    where
        B: Backend,
    {
        ui::demo(f, &self.state);
    }
    pub fn mode(&self) -> state::InputMode {
        self.state.input_mode
    }
    pub fn tick(&mut self) -> Result<AppLifecyle> {
        match self.events.next()? {
            events::Event::Input(input) => {
                // bail early if we ^C
                if input == self.config.keys().exit_key() {
                    return Ok(AppLifecyle::Quit);
                }

                match self.mode() {
                    state::InputMode::Normal => match input {
                        Key::Char('e') => {
                            self.dispatch(events::Event::ChangeInputMode(
                                state::InputMode::Editing,
                            ));
                        }
                        _ => {}
                    },
                    state::InputMode::Editing => match input {
                        Key::Char('\n') => {
                            self.dispatch(events::Event::SubmitInput);
                        }
                        Key::Char(c) => {
                            self.dispatch(events::Event::InputPush(c));
                        }
                        Key::Backspace => {
                            self.dispatch(events::Event::InputPop);
                        }
                        Key::Esc => {
                            self.dispatch(events::Event::ChangeInputMode(state::InputMode::Normal));
                        }
                        _ => {}
                    },
                }
            }
            events::Event::AppActionWrapper(action) => match action {
                events::AppActions::Update(u) => self.state.banner = u,
            },
            _ => {}
        }
        Ok(AppLifecyle::Continue)
    }
}
