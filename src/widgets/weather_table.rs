use chrono::{Local, NaiveDate, NaiveDateTime, Timelike};
use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Cell, Row, Table, Widget},
};

use crate::data::weather::{self, Weather};

#[derive(Default)]
pub struct WeatherTable {
    data: Vec<Weather>,
}

impl Widget for WeatherTable {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if self.data.len() == 0 {
            return;
        }
        let header = Row::new(vec!["Time", "Weather", "Temperature", "Precipitation"]);

        let now = Local::now().naive_local();
        let rows = self.data.iter().map(|i| {
            let (desc, emoji) = weather::get_weather_description(i.weather_code);
            let time = i.date_time;
            let row_style = if time.date() == now.date() && time.hour() == now.hour() {
                Style::new().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::new()
            };
            Row::new(vec![
                Cell::from(format!(
                    "{:>8}",
                    Self::parse_hour(i.date_time.format("%Y-%m-%dT%H:%M").to_string())
                        .unwrap_or_default()
                )),
                Cell::from(format!("{} {}", emoji, desc)),
                Cell::from(format!("{:.1}°F", i.temp)),
                Self::render_precip_bar(i.precip as u8),
            ])
            .style(row_style)
        });

        let widths = [
            Constraint::Percentage(20),
            Constraint::Percentage(35),
            Constraint::Percentage(25),
            Constraint::Percentage(20),
        ];

        let table = Table::new(rows, widths)
            .header(header.style(Style::new().bold()))
            .column_spacing(1)
            .block(Block::new().title("Hourly"));

        Widget::render(table, area, buf);
    }
}

impl WeatherTable {
    pub fn new(weather: Vec<Weather>) -> Self {
        Self { data: weather }
    }

    fn parse_hour(time: String) -> Option<String> {
        let (_, time_part) = time.split_once('T')?;
        let hour_str = &time_part[0..2];
        let hour = hour_str.parse::<u8>().ok()?;

        let suffix = if hour < 12 { "AM" } else { "PM" };

        let hour_12 = match hour {
            0 => 12,
            1..=12 => hour,
            _ => hour - 12,
        };

        let formatted = format!("{} {}", hour_12, suffix);

        Some(formatted)
    }

    fn render_precip_bar(pct: u8) -> Cell<'static> {
        let width = 10;
        let filled = (pct as usize * width) / 100;
        let empty_len = width - filled;
        let filled = Span::styled("█".repeat(filled), Style::new().fg(Color::Blue));
        let empty = Span::raw(" ".repeat(empty_len));

        let gauge = Line::from(vec![
            Span::raw("["),
            filled,
            empty,
            Span::raw("]"),
            Span::raw(format!(" {}%", pct)),
        ]);

        Cell::from(gauge)
    }
}
