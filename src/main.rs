mod app;
mod client;
mod config;
mod events;
mod state;
mod ui;

use crate::app::App;
use anyhow::Result;
use std::io;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};
use youtube_api::{auth::stdio_login, YoutubeApi};

#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(c) = load_config() {
        let api = YoutubeApi::new(c.api_key());
        let mut app = App::with_config(api, c);
        start_ui(&mut app)
    } else {
        panic!("Could not create config")
    }
}

fn load_config() -> Result<config::Config> {
    Ok(config::Config::default())
}

fn start_ui(app: &mut App) -> Result<()> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

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
