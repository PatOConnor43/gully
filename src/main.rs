mod app;
mod config;
mod events;
mod state;
mod ui;

use crate::app::App;
use anyhow::Result;
use std::io;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};
use youtube_api;

#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(c) = load_config() {
        start_ui(c)
    } else {
        panic!("Could not create config")
    }
}

fn load_config() -> Result<config::Config> {
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
        terminal.draw(|f| app.draw(f))?;

        // Handle input
        match app.tick() {
            Ok(app::AppLifecyle::Continue) => {}
            Ok(app::AppLifecyle::Quit) => {
                break;
            }
            Err(e) => panic!(e),
        }
    }
    Ok(())
}
