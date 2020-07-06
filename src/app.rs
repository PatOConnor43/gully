use crate::config;
use crate::events;
use crate::state;
use crate::ui;
use anyhow::Result;
use termion::event::Key;
use tui::{backend::Backend, Frame};

pub enum AppLifecyle {
    Continue,
    Quit,
}

/// App holds the state of the application
#[derive(Default)]
pub struct App {
    config: config::Config,
    events: events::Events,
    state: state::State,
}

impl App {
    pub fn with_config(c: config::Config) -> Self {
        App {
            config: c,
            events: events::Events::default(),
            state: state::State::default(),
        }
    }
    pub fn dispatch(&mut self, e: events::Event<Key>) {
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
        if let events::Event::Input(input) = self.events.next()? {
            match self.mode() {
                state::InputMode::Normal => match input {
                    Key::Char('e') => {
                        self.dispatch(events::Event::ChangeInputMode(state::InputMode::Editing));
                    }
                    Key::Char('q') => {
                        return Ok(AppLifecyle::Quit);
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
        Ok(AppLifecyle::Continue)
    }
}
