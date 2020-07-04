use crate::config;
/// App holds the state of the application
pub struct App {
    config: config::Config,
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded messages
    pub messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            config: config::Config::default(),
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

impl App {
    pub fn with_config(c: config::Config) -> Self {
        App {
            config: c,
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

pub enum InputMode {
    Normal,
    Editing,
}
