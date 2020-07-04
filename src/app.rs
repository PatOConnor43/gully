use crate::config;
use crate::events;
use crate::state;
use anyhow::Result;
use termion::event::Key;
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
    pub fn next_event(&self) -> Result<events::Event<Key>> {
        self.events
            .next()
            .map_err(|_| anyhow::anyhow!("receive error for event"))
    }
    pub fn mode(&self) -> state::InputMode {
        self.state.input_mode
    }
    pub fn state(&self) -> &state::State {
        &self.state
    }
}
