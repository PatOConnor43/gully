use std::time::Duration;
use termion::event::Key;

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub api_key: String,
    pub behavior: BehaviorConfig,
    pub keys: KeyConfig,
}

impl Default for Config {
    fn default() -> Self {
        let key = option_env!("YOUTUBE_API_KEY");
        Self {
            api_key: key.unwrap_or("test").to_owned(),
            behavior: BehaviorConfig::default(),
            keys: KeyConfig::default(),
        }
    }
}

impl Config {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BehaviorConfig {
    pub tick_rate: Duration,
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl BehaviorConfig {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyConfig {
    pub exit_key: Key,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            exit_key: Key::Ctrl('c'),
        }
    }
}

impl KeyConfig {}
