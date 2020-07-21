use crate::state::{InputMode, State};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Text},
    Frame,
};
use unicode_width::UnicodeWidthStr;
const BANNER: &'static str = r#"
                             
    _____  ___   _ 
   / _ \ \/ / | | |
  | (_) >  <| |_| |
   \___/_/\_\\__, |
              |___/ 

               "#;

pub fn demo<B>(f: &mut Frame<B>, state: &State)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let messages = [Text::raw(BANNER), Text::raw(&state.banner)];

    let messages = Paragraph::new(messages.iter())
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center);
    f.render_widget(messages, chunks[0]);

    let msg = match state.input_mode {
        InputMode::Normal => "Press q to exit, e to start editing.",
        InputMode::Editing => "Press Esc to stop editing, Enter to record the message",
    };
    let text = [Text::raw(msg)];
    let help_message = Paragraph::new(text.iter());
    f.render_widget(help_message, chunks[1]);

    let text = [Text::raw(state.input.clone())];
    let input = Paragraph::new(text.iter())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[2]);
    match state.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            //// Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[2].x + state.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[2].y + 1,
            );
        }
    }
}
