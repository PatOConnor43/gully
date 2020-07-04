mod app;
mod config;
mod state;
mod util;

use crate::app::App;
use crate::state::{InputMode, State};
use crate::util::event::{Event, Events};
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
async fn main() -> Result<(), Box<dyn Error>> {
    if let Ok(c) = load_config() {
        start_ui(c)
    } else {
        panic!("Could not create config")
    }
}

fn load_config() -> Result<config::Config, Box<dyn Error>> {
    Ok(config::Config::default())
}

fn start_ui(c: config::Config) -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut events = Events::new();

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

            let text = [Text::raw("placeholder")];
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
        if let Event::Input(input) = events.next()? {
            match app.state().input_mode {
                InputMode::Normal => match input {
                    Key::Char('e') => {
                        app.state().input_mode = InputMode::Editing;
                        events.disable_exit_key();
                    }
                    Key::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                InputMode::Editing => match input {
                    Key::Char('\n') => {
                        app.state()
                            .messages
                            .push(app.state().input.drain(..).collect());
                    }
                    Key::Char(c) => {
                        app.state().input.push(c);
                    }
                    Key::Backspace => {
                        app.state().input.pop();
                    }
                    Key::Esc => {
                        app.state().input_mode = InputMode::Normal;
                        events.enable_exit_key();
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(())
}
