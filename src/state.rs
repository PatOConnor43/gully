#[derive(Clone, Debug)]
pub struct State {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded messages
    pub messages: Vec<String>,
    pub banner: String,
    pub titles: Vec<String>,
}
impl Default for State {
    fn default() -> Self {
        State {
            banner: String::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            titles: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputMode {
    Normal,
    Editing,
}
