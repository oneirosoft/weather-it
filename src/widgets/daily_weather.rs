use chrono::{Datelike, NaiveDate};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::data::weather::{OpenMeteoDaily, get_weather_description};

#[derive(Debug, Default, Clone)]
pub struct DailyWeather {
    data: OpenMeteoDaily,
    selected_date: String,
}

impl Widget for DailyWeather {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let horizontal =
            Layout::horizontal((0..self.data.date.len()).map(|_| Constraint::Fill(2))).spacing(2);

        let rows = Layout::vertical([Constraint::Length(6)])
            .spacing(1)
            .split(area);

        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            fn calc_cell(rect: Rect) -> Rect {
                Rect {
                    x: rect.x + 1,
                    y: rect.y + 1,
                    width: rect.width - 2,
                    height: rect.height - 2,
                }
            }
            let cell_layout = Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(calc_cell(cell));

            let date_str =
                format_date_with_suffix(self.data.date[i].as_str()).unwrap_or("".to_string());
            let block = if self.selected_date == self.data.date[i]
                || (self.selected_date == "" && self.data.date[0] == self.data.date[i])
            {
                Block::default().style(Style::new().fg(Color::LightBlue))
            } else {
                Block::default()
            };

            block
                .borders(Borders::all())
                .title(date_str)
                .render(cell, buf);
            let (weather_desc, weather_emoji) = get_weather_description(self.data.weather_code[i]);

            Paragraph::new(format!("{} {}", weather_emoji, weather_desc))
                .render(cell_layout[0], buf);

            Paragraph::new(format!("🌡️ {:.1}°F", self.data.temperature_2m_max[i]))
                .render(cell_layout[1], buf);

            let feels_temp = self.data.apparent_temperature_max[i];
            let feels_emoji = if feels_temp < 50.0 { "🥶" } else { "🥵" };

            Paragraph::new(format!(
                "{} {:.1}°F",
                feels_emoji, self.data.apparent_temperature_max[i]
            ))
            .render(cell_layout[2], buf);

            Paragraph::new(format!(
                "☔️ {}%",
                self.data.precipitation_probability_max[i]
            ))
            .render(cell_layout[3], buf);
        }
    }
}

impl DailyWeather {
    pub fn data(&mut self, data: OpenMeteoDaily) {
        if self.data.date.len() == 0 {
            self.selected_date = self
                .data
                .date
                .first()
                .map_or("".to_string(), |date| date.to_string());
        }
        self.data = data;
    }

    pub fn select_next(&mut self) {
        let next = self
            .data
            .date
            .iter()
            .position(|date| date == &self.selected_date)
            .unwrap_or(0)
            + 1;
        let index = if next == self.data.date.len() {
            0
        } else {
            next
        };

        self.selected_date = self.data.date[index].clone();
    }

    pub fn select_previous(&mut self) {
        let prev = self
            .data
            .date
            .iter()
            .position(|date| date == &self.selected_date)
            .unwrap_or(0);
        let index = if prev == 0 {
            self.data.date.len() - 1
        } else {
            prev - 1
        };

        self.selected_date = self.data.date[index].clone();
    }

    pub fn selected(self) -> String {
        self.selected_date.clone()
    }
}

fn format_date_with_suffix(input: &str) -> Option<String> {
    let date = NaiveDate::parse_from_str(input, "%Y-%m-%d").ok()?;
    let weekday = date.format("%a").to_string(); // "Sun"
    let month = date.format("%b").to_string(); // "Jun"
    let day = date.day();
    let suffix = match day {
        11 | 12 | 13 => "th",
        _ => match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };

    Some(format!("{}, {} {}{}", weekday, month, day, suffix))
}
