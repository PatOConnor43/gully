use crate::config;
use crate::state;
/// App holds the state of the application
pub struct App {
    config: config::Config,

    state: state::State,
}

impl Default for App {
    fn default() -> App {
        App {
            config: config::Config::default(),
            state: state::State::default(),
        }
    }
}

impl App {
    pub fn with_config(c: config::Config) -> Self {
        App {
            config: c,
            state: state::State::default(),
        }
    }
    pub fn state(&self) -> &state::State {
        &self.state
    }
}
