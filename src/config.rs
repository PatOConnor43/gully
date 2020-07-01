
use std::time::Duration;
use termion::event::Key;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Config {
    behavior: BehaviorConfig,
    keys: KeyConfig
}

impl Default for Config {
    fn default() -> Self {
        Self {
            behavior: BehaviorConfig::default(),
            keys: KeyConfig::default()
        }
    }
}

impl Config {
    pub fn behavior(&self) -> BehaviorConfig { self.behavior }
    pub fn keys(&self) -> KeyConfig { self.keys }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BehaviorConfig {
    tick_rate: Duration
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            tick_rate: Duration::from_millis(250)
        }
    }
}

impl BehaviorConfig {
    pub fn tick_rate(&self) -> Duration { self.tick_rate }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyConfig {
    exit_key: Key
}

impl Default for KeyConfig {
    fn default() -> Self { Self{exit_key: Key::Char('q')} }
}

impl KeyConfig {
    pub fn exit_key(&self) -> Key { self.exit_key }
}
