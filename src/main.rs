mod app;
mod config;
mod events;
mod models;
mod state;
mod ui;

use crate::app::App;
use crate::events::{AppActions, BackgroundActions};
use crate::models::SearchResponse;
use anyhow::Result;
use std::io;
use std::sync::mpsc;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};
use youtube_api::YoutubeApi;

pub struct BackgroundTaskHandler<T> {
    api: YoutubeApi,
    sender: std::sync::mpsc::Sender<T>,
}

impl BackgroundTaskHandler<AppActions> {
    pub fn new(api: YoutubeApi, sender: std::sync::mpsc::Sender<AppActions>) -> Self {
        Self { api, sender }
    }

    pub async fn handle_event(&self, e: BackgroundActions) {
        match e {
            BackgroundActions::YoutubeQuery(q) => {
                let result = self
                    .api
                    .search(youtube_api::models::SearchRequestBuilder {
                        query: Some(q),
                        channel_id: None,
                    })
                    .await;

                match result {
                    Ok(r) => {
                        let titles = r
                            .items
                            .iter()
                            .map(|i| i.snippet.title.clone())
                            .collect::<Vec<String>>();
                        self.sender
                            .send(AppActions::SearchResponseAction(SearchResponse::new(
                                titles,
                            )));
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn start_background_task_handler(
    rx: std::sync::mpsc::Receiver<BackgroundActions>,
    tx: std::sync::mpsc::Sender<AppActions>,
    api: YoutubeApi,
) {
    let handler = BackgroundTaskHandler::new(api, tx);
    while let Ok(e) = rx.recv() {
        handler.handle_event(e).await;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(c) = load_config() {
        let api = YoutubeApi::new(c.api_key());
        let (app_tx, app_rx) = mpsc::channel::<AppActions>();
        let (background_tx, background_rx) = mpsc::channel::<BackgroundActions>();

        std::thread::spawn(move || {
            start_background_task_handler(background_rx, app_tx, api);
        });
        let mut app = App::new(app_rx, background_tx, c);
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
