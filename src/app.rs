use std::{boxed::Box, error::Error, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
    style::Stylize,
    widgets::Paragraph,
};
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::sleep,
};

use crate::{
    data::weather::OpenMeteoResponse,
    layout::{self, center},
    weather_service::WeatherData,
    widgets::{
        daily_weather::DailyWeather, loader::Loader, search::Search, weather_table::WeatherTable,
    },
};

pub struct App {
    search: Search,
    loader: Loader,
    daily: DailyWeather,
    location_name: Option<String>,
    exit: bool,
    weather: OpenMeteoResponse,
    weather_tx: Sender<WeatherData>,
    loading: bool,
}

impl App {
    pub fn new(weather_tx: Sender<WeatherData>) -> Self {
        Self {
            search: Search::default(),
            daily: DailyWeather::default(),
            location_name: None,
            exit: false,
            weather: OpenMeteoResponse::default(),
            weather_tx,
            loading: false,
            loader: Loader::default(),
        }
    }

    pub async fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        weather_rx: &mut Receiver<WeatherData>,
    ) -> Result<(), Box<dyn Error>> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().await?;
            // Poll for weather updates
            if self.loading {
                self.loader.calc_next();
            }

            if let Ok(Some(weather_data)) =
                tokio::time::timeout(Duration::from_millis(10), weather_rx.recv()).await
            {
                self.update_state(weather_data);
            }
            sleep(Duration::from_millis(10)).await;
        }
        Ok(())
    }

    fn update_state(&mut self, weather_data: WeatherData) {
        self.daily.data(weather_data.weather.daily.clone());
        self.weather = weather_data.weather;
        self.location_name = Some(weather_data.location_name.clone());
        self.loading = false;
        self.loader = Loader::default();
    }

    fn draw(&self, frame: &mut Frame) {
        let app_layout = layout::default_layout(frame.area());
        let centered_search = center(app_layout[0], app_layout[0].width / 3);
        let centered_title = center(app_layout[1], (app_layout[1].width as f32 * 0.8) as u16);
        let centered_daily = center(app_layout[2], (app_layout[2].width as f32 * 0.8) as u16);
        let centered_weather = center(app_layout[3], (app_layout[3].width as f32 * 0.8) as u16);
        let loader_area = Rect {
            x: centered_search.x + centered_search.width.saturating_sub(3),
            y: centered_search.y + centered_search.height.saturating_sub(2),
            width: 1,
            height: 1,
        };
        frame.render_widget(self.search.clone(), centered_search);
        if self.loading {
            frame.render_widget(self.loader.clone(), loader_area);
        }

        if let Some(location_name) = &self.location_name {
            let title = Paragraph::new(location_name.as_str()).bold().centered();
            frame.render_widget(title, centered_title);
        }

        if self.weather.hourly.date_time.len() > 0 {
            frame.render_widget(
                WeatherTable::new(
                    self.weather.hourly.clone(),
                    self.daily.clone().selected().clone(),
                ),
                centered_weather,
            );
        }

        if self.weather.daily.date.len() > 0 {
            frame.render_widget(self.daily.clone(), centered_daily);
        }
    }

    async fn handle_events(&mut self) -> Result<(), Box<dyn Error>> {
        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                Event::Key(key_event) => self.handle_key_event(key_event).await,
                _ => {}
            }
        }

        Ok(())
    }

    async fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event {
            KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                code: KeyCode::Char('c'),
                ..
            } => self.exit = true,
            KeyEvent {
                code: KeyCode::Enter,
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('r'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                let _ = self.update_weather().await;
            }
            KeyEvent {
                code: KeyCode::Tab, ..
            } => self.daily.select_next(),
            KeyEvent {
                code: KeyCode::BackTab,
                ..
            } => self.daily.select_previous(),
            _ => self.search.handle_key_event(key_event),
        }
    }

    async fn update_weather(&mut self) {
        self.loading = true;
        let tx = self.weather_tx.clone();
        let query = self.search.text().to_string();

        tokio::spawn(async move {
            if let Ok(result) = crate::weather_service::dispatch_weather(&query).await {
                let _ = tx.send(result).await;
            }
        });
    }
}
