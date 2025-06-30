mod app;
mod data;
mod layout;
mod weather_service;
mod widgets;

use app::App;
use std::error::Error;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let (tx, mut rx) = mpsc::channel(1);
    let mut app = App::new(tx);
    let app_result = app.run(&mut terminal, &mut rx).await;
    ratatui::restore();
    app_result
}
