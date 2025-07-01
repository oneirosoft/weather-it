use std::rc::Rc;

use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};

pub fn default_layout(area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(6),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area)
}

pub fn center(area: Rect, width: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    area
}
