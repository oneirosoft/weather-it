use chrono::{Local, NaiveDate};
use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Cell, Row, Table, Widget},
};

use crate::data::weather::{self, OpenMeteoHourly};

#[derive(Default)]
pub struct WeatherTable {
    data: OpenMeteoHourly,
    start_date: String,
}

impl Widget for WeatherTable {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if self.data.date_time.len() == 0 {
            return;
        }
        let header = Row::new(vec!["Time", "Weather", "Temperature", "Precipitation"]);
        let start_date = NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d")
            .unwrap_or(Local::now().date_naive());
        let start_index = self
            .data
            .date_time
            .iter()
            .position(|date| {
                let (date_str, _) = date.split_once('T').unwrap();
                let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .unwrap_or(Local::now().date_naive());
                date >= start_date
            })
            .unwrap_or(0);
        let rows = (start_index..start_index + 24).map(|i| {
            let (desc, emoji) = weather::get_weather_description(self.data.weather_code[i]);
            Row::new(vec![
                Cell::from(format!(
                    "{:>8}",
                    Self::parse_hour(self.data.date_time[i].clone()).unwrap_or_default()
                )),
                Cell::from(format!("{} {}", emoji, desc)),
                Cell::from(format!("{:.1}°F", self.data.temperature_2m[i])),
                Self::render_precip_bar(self.data.precipitation_probability[i] as u8),
            ])
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
    pub fn new(hourly: OpenMeteoHourly, start_date: String) -> Self {
        Self {
            data: hourly,
            start_date: start_date,
        }
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
