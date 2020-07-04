mod app;
mod config;
mod events;
mod state;

use crate::app::App;
use crate::events::{Event, Events};
use crate::state::{InputMode, State};
use anyhow::Result;
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, Paragraph, Text},
    Terminal,
};
use unicode_width::UnicodeWidthStr;
use youtube_api;

#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(c) = load_config() {
        start_ui(c)
    } else {
        panic!("Could not create config")
    }
}

fn load_config() -> Result<config::Config, Box<dyn Error>> {
    Ok(config::Config::default())
}

fn start_ui(c: config::Config) -> Result<()> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create default app state
    let mut app = App::with_config(c);

    loop {
        terminal.draw(|f| {
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

            let messages = [Text::raw(
                r#"
                             
  _____  ___   _ 
 / _ \ \/ / | | |
| (_) >  <| |_| |
 \___/_/\_\\__, |
           |___/ 

"#,
            )];

            let messages = Paragraph::new(messages.iter())
                .block(Block::default().borders(Borders::NONE))
                .alignment(Alignment::Center);
            f.render_widget(messages, chunks[0]);

            let msg = match app.state().input_mode {
                InputMode::Normal => "Press q to exit, e to start editing.",
                InputMode::Editing => "Press Esc to stop editing, Enter to record the message",
            };
            let text = [Text::raw(msg)];
            let help_message = Paragraph::new(text.iter());
            f.render_widget(help_message, chunks[1]);

            let text = [Text::raw(app.state().input.clone())];
            let input = Paragraph::new(text.iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input, chunks[2]);
            match app.state().input_mode {
                InputMode::Normal =>
                    // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                    {}

                InputMode::Editing => {
                    //// Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                    f.set_cursor(
                        // Put cursor past the end of the input text
                        chunks[2].x + app.state().input.width() as u16 + 1,
                        // Move one line down, from the border to the input line
                        chunks[2].y + 1,
                    );
                }
            }
        })?;

        // Handle input
        if let Event::Input(input) = app.next_event()? {
            match app.mode() {
                InputMode::Normal => match input {
                    Key::Char('e') => {
                        app.dispatch(events::Event::ChangeInputMode(InputMode::Editing));
                    }
                    Key::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                InputMode::Editing => match input {
                    Key::Char('\n') => {
                        app.dispatch(events::Event::SubmitInput);
                    }
                    Key::Char(c) => {
                        app.dispatch(events::Event::InputPush(c));
                    }
                    Key::Backspace => {
                        app.dispatch(events::Event::InputPop);
                    }
                    Key::Esc => {
                        app.dispatch(events::Event::ChangeInputMode(InputMode::Normal));
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(())
}
