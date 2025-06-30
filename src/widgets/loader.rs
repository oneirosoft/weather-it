use ratatui::{
    style::{Color, Style},
    widgets::{StatefulWidget, Widget},
};
use throbber_widgets_tui::{Throbber, ThrobberState};

#[derive(Default, Clone)]
pub struct Loader {
    tick: usize,
    state: ThrobberState,
}

impl Widget for Loader {
    fn render(mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let throbber = Throbber::default()
            .throbber_set(throbber_widgets_tui::BRAILLE_EIGHT_DOUBLE)
            .style(Style::new().fg(Color::Cyan));

        StatefulWidget::render(throbber, area, buf, &mut self.state);
    }
}

impl Loader {
    pub fn calc_next(&mut self) {
        self.tick += 1;

        if self.tick % 6 == 0 {
            self.state.calc_next();
        }
    }
}
