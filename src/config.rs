use std::time::Duration;
use termion::event::Key;

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    api_key: String,
    behavior: BehaviorConfig,
    keys: KeyConfig,
}

impl Default for Config {
    fn default() -> Self {
        let key = env!("YOUTUBE_API_KEY");
        Self {
            api_key: key.to_owned(),
            behavior: BehaviorConfig::default(),
            keys: KeyConfig::default(),
        }
    }
}

impl Config {
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
    pub fn behavior(&self) -> BehaviorConfig {
        self.behavior
    }
    pub fn keys(&self) -> KeyConfig {
        self.keys
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BehaviorConfig {
    tick_rate: Duration,
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl BehaviorConfig {
    pub fn tick_rate(&self) -> Duration {
        self.tick_rate
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyConfig {
    exit_key: Key,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            exit_key: Key::Ctrl('c'),
        }
    }
}

impl KeyConfig {
    pub fn exit_key(&self) -> Key {
        self.exit_key
    }
}
