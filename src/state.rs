pub struct State {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded messages
    pub messages: Vec<String>,
}
impl Default for State {
    fn default() -> Self {
        State {
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
